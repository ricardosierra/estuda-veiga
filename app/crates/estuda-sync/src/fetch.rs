//! Coleta HTTP eficiente e segura: requisições condicionais (ETag/Last-Modified/304),
//! TTL local, orçamento (requisições, bytes, duração), retries limitados com backoff,
//! rate limit, cancelamento, expiração de sessão e métricas.
//!
//! O transporte real (reqwest ou `fetch` via CDP) implementa [`Transport`] fora deste crate;
//! `estuda-core::download` já faz condicional para PDFs e pode ser adaptado. Aqui só entra
//! GET, e toda URL passa por [`ReadOnlyPolicy`] antes de sair.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;
use thiserror::Error;

use crate::policy::{Blocker, HttpMethod, ReadOnlyPolicy};
use crate::text::sha256_hex;

pub trait Clock {
    fn now_ms(&self) -> u64;
    fn sleep_ms(&self, ms: u64);
}

pub struct SystemClock {
    origin: std::time::Instant,
}

impl SystemClock {
    pub fn new() -> Self {
        Self {
            origin: std::time::Instant::now(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        self.origin.elapsed().as_millis() as u64
    }
    fn sleep_ms(&self, ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}

#[derive(Debug, Clone, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Validators {
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Debug)]
pub struct Request<'a> {
    pub url: &'a str,
    pub validators: Option<&'a Validators>,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Default)]
pub struct Response {
    pub status: u16,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub body: Vec<u8>,
    /// Bytes realmente trafegados (cabeçalhos + corpo comprimido), se o transporte souber.
    pub wire_bytes: Option<u64>,
    /// URL final após redirecionamentos, se houve.
    pub final_url: Option<String>,
    pub retry_after_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TransportError {
    #[error("tempo esgotado")]
    Timeout,
    #[error("falha de rede: {0}")]
    Network(String),
}

pub trait Transport {
    fn send(&self, request: &Request<'_>) -> Result<Response, TransportError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FetchError {
    #[error("cancelado")]
    Cancelled,
    #[error("bloqueado pela política somente-leitura: {0}")]
    Blocked(Blocker),
    #[error("limite excedido: {0}")]
    LimitExceeded(&'static str),
    #[error("sessão expirada: faça login manualmente; nada será renovado automaticamente")]
    SessionExpired,
    #[error("HTTP {0}")]
    Http(u16),
    #[error("tempo esgotado após retries")]
    Timeout,
    #[error("falha de rede: {0}")]
    Network(String),
    #[error("resposta 304 sem validadores locais")]
    UnexpectedNotModified,
    #[error("política inválida: {0}")]
    InvalidPolicy(&'static str),
}

#[derive(Debug, Clone)]
pub struct FetchPolicy {
    pub max_requests: u32,
    pub max_total_bytes: u64,
    pub max_body_bytes: u64,
    pub max_duration_ms: u64,
    pub max_retries: u32,
    pub backoff_base_ms: u64,
    pub backoff_cap_ms: u64,
    /// Intervalo mínimo entre requisições (rate limit configurável).
    pub min_interval_ms: u64,
    pub timeout_ms: u64,
    /// Dentro deste prazo uma entrada em cache dispensa até a requisição condicional.
    pub fresh_ttl_ms: u64,
}

impl Default for FetchPolicy {
    fn default() -> Self {
        Self {
            max_requests: 500,
            max_total_bytes: 200 * 1024 * 1024,
            max_body_bytes: 60 * 1024 * 1024,
            max_duration_ms: 30 * 60 * 1000,
            max_retries: 3,
            backoff_base_ms: 500,
            backoff_cap_ms: 8_000,
            min_interval_ms: 250,
            timeout_ms: 45_000,
            fresh_ttl_ms: 5 * 60 * 1000,
        }
    }
}

impl FetchPolicy {
    pub fn validate(&self) -> Result<(), FetchError> {
        if self.max_requests == 0
            || self.max_total_bytes == 0
            || self.max_body_bytes == 0
            || self.max_duration_ms == 0
        {
            return Err(FetchError::InvalidPolicy(
                "limites devem ser maiores que zero",
            ));
        }
        if self.max_retries > 5 {
            return Err(FetchError::InvalidPolicy("no máximo 5 retries"));
        }
        if !(1_000..=120_000).contains(&self.timeout_ms) {
            return Err(FetchError::InvalidPolicy(
                "timeout deve ficar entre 1 s e 120 s",
            ));
        }
        if self.backoff_cap_ms < self.backoff_base_ms {
            return Err(FetchError::InvalidPolicy(
                "teto do backoff menor que a base",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct Counters {
    pub requests: u64,
    pub bytes: u64,
}

/// Métricas da execução. `chromium` fica `None` até alguém medir: o tráfego do Chromium
/// (recursos da página, scripts) é **adicional** ao controlado pelo app e não é estimado aqui.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SyncMetrics {
    pub requests_executed: u64,
    pub requests_avoided_by_cache: u64,
    pub responses_304: u64,
    pub bytes_transferred: u64,
    pub bytes_avoided_estimated: u64,
    pub elapsed_ms: u64,
    pub failures: u64,
    pub retries: u64,
    pub items_updated: u64,
    pub items_unchanged: u64,
    pub items_pending: u64,
    pub chromium: Option<Counters>,
}

impl SyncMetrics {
    pub fn record_chromium_traffic(&mut self, measured: Counters) {
        self.chromium = Some(measured);
    }

    pub fn report_lines(&self) -> Vec<String> {
        let chromium = match self.chromium {
            Some(c) => format!("{} requisições, {} bytes (medido)", c.requests, c.bytes),
            None => "não medido (tráfego do Chromium é adicional e não entra nos números acima)"
                .to_owned(),
        };
        vec![
            format!("Requisições do app: {}", self.requests_executed),
            format!("Evitadas pelo cache: {}", self.requests_avoided_by_cache),
            format!("Respostas 304: {}", self.responses_304),
            format!("Bytes transferidos: {}", self.bytes_transferred),
            format!(
                "Bytes evitados (estimativa): {}",
                self.bytes_avoided_estimated
            ),
            format!("Tempo total: {} ms", self.elapsed_ms),
            format!("Falhas: {} · retries: {}", self.failures, self.retries),
            format!(
                "Itens: {} atualizados · {} sem alteração · {} pendentes",
                self.items_updated, self.items_unchanged, self.items_pending
            ),
            format!("Chromium: {chromium}"),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedEntry {
    pub validators: Validators,
    pub content_hash: String,
    pub len: u64,
    pub fetched_at_ms: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FetchOutcome {
    /// Dentro do TTL: nenhuma requisição saiu.
    Fresh { content_hash: String, len: u64 },
    /// 304: o conteúdo local continua válido.
    NotModified { content_hash: String, len: u64 },
    Fetched {
        body: Vec<u8>,
        content_hash: String,
        changed: bool,
    },
}

pub struct Fetcher<T: Transport, C: Clock> {
    transport: T,
    clock: C,
    policy: FetchPolicy,
    guard: ReadOnlyPolicy,
    cancel: CancelToken,
    cache: HashMap<String, CachedEntry>,
    pub metrics: SyncMetrics,
    started_ms: u64,
    last_send_ms: Option<u64>,
}

impl<T: Transport, C: Clock> Fetcher<T, C> {
    pub fn new(
        transport: T,
        clock: C,
        policy: FetchPolicy,
        guard: ReadOnlyPolicy,
        cancel: CancelToken,
    ) -> Result<Self, FetchError> {
        policy.validate()?;
        let started_ms = clock.now_ms();
        Ok(Self {
            transport,
            clock,
            policy,
            guard,
            cancel,
            cache: HashMap::new(),
            metrics: SyncMetrics::default(),
            started_ms,
            last_send_ms: None,
        })
    }

    /// Hidrata validadores já conhecidos (ex.: lidos do banco quando o banco for aprovado).
    pub fn seed_cache(&mut self, url: &str, entry: CachedEntry) {
        self.cache.insert(url.to_owned(), entry);
    }

    pub fn cached(&self, url: &str) -> Option<&CachedEntry> {
        self.cache.get(url)
    }

    fn elapsed(&self) -> u64 {
        self.clock.now_ms().saturating_sub(self.started_ms)
    }

    fn check_budget(&self) -> Result<(), FetchError> {
        if self.cancel.is_cancelled() {
            return Err(FetchError::Cancelled);
        }
        if self.elapsed() > self.policy.max_duration_ms {
            return Err(FetchError::LimitExceeded("duração máxima"));
        }
        if self.metrics.requests_executed >= u64::from(self.policy.max_requests) {
            return Err(FetchError::LimitExceeded("número de requisições"));
        }
        if self.metrics.bytes_transferred >= self.policy.max_total_bytes {
            return Err(FetchError::LimitExceeded("bytes totais"));
        }
        Ok(())
    }

    /// Dorme em fatias de ≤100 ms para o cancelamento ser quase imediato.
    fn sleep_cancellable(&self, ms: u64) -> Result<(), FetchError> {
        let mut remaining = ms;
        while remaining > 0 {
            if self.cancel.is_cancelled() {
                return Err(FetchError::Cancelled);
            }
            let slice = remaining.min(100);
            self.clock.sleep_ms(slice);
            remaining -= slice;
        }
        if self.cancel.is_cancelled() {
            Err(FetchError::Cancelled)
        } else {
            Ok(())
        }
    }

    fn backoff_ms(&self, attempt: u32, retry_after: Option<u64>) -> u64 {
        let exp = self
            .policy
            .backoff_base_ms
            .saturating_mul(1u64 << attempt.min(16));
        exp.max(retry_after.unwrap_or(0))
            .min(self.policy.backoff_cap_ms)
    }

    pub fn fetch(&mut self, url: &str) -> Result<FetchOutcome, FetchError> {
        let result = self.fetch_inner(url);
        self.metrics.elapsed_ms = self.elapsed();
        if let Err(e) = &result
            && !matches!(e, FetchError::Cancelled)
        {
            self.metrics.failures += 1;
        }
        result
    }

    fn fetch_inner(&mut self, url: &str) -> Result<FetchOutcome, FetchError> {
        self.check_budget()?;
        self.guard
            .check_request(HttpMethod::Get, url)
            .map_err(FetchError::Blocked)?;

        if let Some(entry) = self.cache.get(url) {
            let age = self.clock.now_ms().saturating_sub(entry.fetched_at_ms);
            if age < self.policy.fresh_ttl_ms {
                self.metrics.requests_avoided_by_cache += 1;
                self.metrics.bytes_avoided_estimated += entry.len;
                return Ok(FetchOutcome::Fresh {
                    content_hash: entry.content_hash.clone(),
                    len: entry.len,
                });
            }
        }

        let mut attempt = 0u32;
        loop {
            self.check_budget()?;
            if let Some(last) = self.last_send_ms {
                let wait = (last + self.policy.min_interval_ms).saturating_sub(self.clock.now_ms());
                if wait > 0 {
                    self.sleep_cancellable(wait)?;
                }
            }
            let validators = self.cache.get(url).map(|e| e.validators.clone());
            let request = Request {
                url,
                validators: validators
                    .as_ref()
                    .filter(|v| v.etag.is_some() || v.last_modified.is_some()),
                timeout_ms: self.policy.timeout_ms,
            };
            self.last_send_ms = Some(self.clock.now_ms());
            self.metrics.requests_executed += 1;
            let sent = self.transport.send(&request);

            let retry_after;
            let failure = match sent {
                Ok(response) => {
                    self.metrics.bytes_transferred +=
                        response.wire_bytes.unwrap_or(response.body.len() as u64);
                    match self.classify(url, response)? {
                        Classified::Done(outcome) => return Ok(outcome),
                        Classified::Retry { status, after } => {
                            retry_after = after;
                            FetchError::Http(status)
                        }
                    }
                }
                Err(TransportError::Timeout) => {
                    retry_after = None;
                    FetchError::Timeout
                }
                Err(TransportError::Network(m)) => {
                    retry_after = None;
                    FetchError::Network(m)
                }
            };
            if attempt >= self.policy.max_retries {
                return Err(failure);
            }
            let wait = self.backoff_ms(attempt, retry_after);
            attempt += 1;
            self.metrics.retries += 1;
            self.sleep_cancellable(wait)?;
        }
    }

    fn classify(&mut self, url: &str, response: Response) -> Result<Classified, FetchError> {
        if let Some(final_url) = &response.final_url {
            match self.guard.check_request(HttpMethod::Get, final_url) {
                Err(Blocker::AuthResource) => return Err(FetchError::SessionExpired),
                Err(other) => return Err(FetchError::Blocked(other)),
                Ok(()) => {}
            }
        }
        match response.status {
            200..=299 => {
                if response.body.len() as u64 > self.policy.max_body_bytes {
                    return Err(FetchError::LimitExceeded("tamanho da resposta"));
                }
                let content_hash = sha256_hex(&response.body);
                let changed = self
                    .cache
                    .get(url)
                    .is_none_or(|e| e.content_hash != content_hash);
                if changed {
                    self.metrics.items_updated += 1;
                } else {
                    self.metrics.items_unchanged += 1;
                }
                self.cache.insert(
                    url.to_owned(),
                    CachedEntry {
                        validators: Validators {
                            etag: response.etag,
                            last_modified: response.last_modified,
                        },
                        content_hash: content_hash.clone(),
                        len: response.body.len() as u64,
                        fetched_at_ms: self.clock.now_ms(),
                    },
                );
                Ok(Classified::Done(FetchOutcome::Fetched {
                    body: response.body,
                    content_hash,
                    changed,
                }))
            }
            304 => {
                let Some(entry) = self.cache.get_mut(url) else {
                    return Err(FetchError::UnexpectedNotModified);
                };
                entry.fetched_at_ms = self.clock.now_ms();
                self.metrics.responses_304 += 1;
                self.metrics.items_unchanged += 1;
                self.metrics.bytes_avoided_estimated += entry.len;
                Ok(Classified::Done(FetchOutcome::NotModified {
                    content_hash: entry.content_hash.clone(),
                    len: entry.len,
                }))
            }
            401 | 403 => Err(FetchError::SessionExpired),
            429 | 500 | 502 | 503 | 504 => Ok(Classified::Retry {
                status: response.status,
                after: response.retry_after_ms,
            }),
            other => Err(FetchError::Http(other)),
        }
    }
}

enum Classified {
    Done(FetchOutcome),
    Retry { status: u16, after: Option<u64> },
}

// ---------------------------------------------------------------------------------------
// Paginação controlada
// ---------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct PageLimits {
    pub max_pages: u32,
    pub max_records: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paged<T> {
    pub items: Vec<T>,
    /// `false` se parou por limite com mais páginas disponíveis: resultado parcial.
    pub complete: bool,
    pub pages: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PageError<E> {
    Cancelled,
    Source(E),
}

/// `next_page(n)` devolve `(registros, há_próxima)`. Páginas além do limite não são pedidas.
pub fn collect_pages<T, E>(
    limits: PageLimits,
    cancel: &CancelToken,
    mut next_page: impl FnMut(u32) -> Result<(Vec<T>, bool), E>,
) -> Result<Paged<T>, PageError<E>> {
    let mut items = Vec::new();
    let mut pages = 0;
    loop {
        if cancel.is_cancelled() {
            return Err(PageError::Cancelled);
        }
        if pages >= limits.max_pages {
            return Ok(Paged {
                items,
                complete: false,
                pages,
            });
        }
        let (mut batch, has_next) = next_page(pages + 1).map_err(PageError::Source)?;
        pages += 1;
        let room = limits.max_records.saturating_sub(items.len());
        if batch.len() > room {
            batch.truncate(room);
            items.extend(batch);
            return Ok(Paged {
                items,
                complete: false,
                pages,
            });
        }
        items.extend(batch);
        if !has_next {
            return Ok(Paged {
                items,
                complete: true,
                pages,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::rc::Rc;

    use super::*;

    const URL: &str = "https://uva.instructure.com/api/v1/courses";

    #[derive(Clone, Default)]
    struct FakeClock {
        now: Rc<Cell<u64>>,
        slept: Rc<RefCell<Vec<u64>>>,
        cancel_after_sleep: Rc<RefCell<Option<CancelToken>>>,
    }
    impl Clock for FakeClock {
        fn now_ms(&self) -> u64 {
            self.now.get()
        }
        fn sleep_ms(&self, ms: u64) {
            self.now.set(self.now.get() + ms);
            self.slept.borrow_mut().push(ms);
            if let Some(token) = self.cancel_after_sleep.borrow().as_ref() {
                token.cancel();
            }
        }
    }

    #[derive(Default)]
    struct FakeTransport {
        script: RefCell<VecDeque<Result<Response, TransportError>>>,
        seen: RefCell<Vec<(String, Option<Validators>, u64)>>,
        clock: Option<FakeClock>,
        advance_per_call: u64,
    }
    impl FakeTransport {
        fn with(script: Vec<Result<Response, TransportError>>) -> Self {
            Self {
                script: RefCell::new(script.into()),
                ..Default::default()
            }
        }
    }
    impl Transport for &FakeTransport {
        fn send(&self, request: &Request<'_>) -> Result<Response, TransportError> {
            if let Some(c) = &self.clock {
                c.now.set(c.now.get() + self.advance_per_call);
            }
            self.seen.borrow_mut().push((
                request.url.to_owned(),
                request.validators.cloned(),
                request.timeout_ms,
            ));
            self.script
                .borrow_mut()
                .pop_front()
                .expect("script esgotado")
        }
    }

    fn ok(body: &str, etag: Option<&str>) -> Result<Response, TransportError> {
        Ok(Response {
            status: 200,
            body: body.as_bytes().to_vec(),
            etag: etag.map(Into::into),
            ..Default::default()
        })
    }
    fn status(code: u16) -> Result<Response, TransportError> {
        Ok(Response {
            status: code,
            ..Default::default()
        })
    }

    fn fetcher<'a>(
        t: &'a FakeTransport,
        clock: &FakeClock,
        policy: FetchPolicy,
    ) -> Fetcher<&'a FakeTransport, FakeClock> {
        Fetcher::new(
            t,
            clock.clone(),
            policy,
            ReadOnlyPolicy::uva_default(),
            CancelToken::new(),
        )
        .unwrap()
    }

    fn quick() -> FetchPolicy {
        FetchPolicy {
            min_interval_ms: 0,
            fresh_ttl_ms: 0,
            ..Default::default()
        }
    }

    #[test]
    fn conditional_request_sends_validators_and_304_reuses_cache() {
        let t = FakeTransport::with(vec![ok("conteudo", Some("\"v1\"")), status(304)]);
        let clock = FakeClock::default();
        let mut f = fetcher(&t, &clock, quick());
        let first = f.fetch(URL).unwrap();
        let hash = match first {
            FetchOutcome::Fetched {
                changed: true,
                content_hash,
                ..
            } => content_hash,
            other => panic!("{other:?}"),
        };
        assert_eq!(
            f.fetch(URL).unwrap(),
            FetchOutcome::NotModified {
                content_hash: hash,
                len: 8
            }
        );
        let seen = t.seen.borrow();
        assert_eq!(seen[0].1, None);
        assert_eq!(seen[1].1.as_ref().unwrap().etag.as_deref(), Some("\"v1\""));
        assert_eq!(
            (
                f.metrics.responses_304,
                f.metrics.bytes_avoided_estimated,
                f.metrics.requests_executed
            ),
            (1, 8, 2)
        );
        assert_eq!((f.metrics.items_updated, f.metrics.items_unchanged), (1, 1));
    }

    #[test]
    fn fresh_cache_avoids_the_request_entirely_and_expired_cache_revalidates() {
        let t = FakeTransport::with(vec![ok("abc", Some("\"e\"")), status(304)]);
        let clock = FakeClock::default();
        let mut f = fetcher(
            &t,
            &clock,
            FetchPolicy {
                min_interval_ms: 0,
                fresh_ttl_ms: 1_000,
                ..Default::default()
            },
        );
        f.fetch(URL).unwrap();
        clock.now.set(500);
        assert!(matches!(f.fetch(URL).unwrap(), FetchOutcome::Fresh { .. }));
        assert_eq!(f.metrics.requests_executed, 1);
        assert_eq!(f.metrics.requests_avoided_by_cache, 1);
        clock.now.set(2_000);
        assert!(matches!(
            f.fetch(URL).unwrap(),
            FetchOutcome::NotModified { .. }
        ));
        assert_eq!(t.seen.borrow().len(), 2);
    }

    #[test]
    fn changed_body_after_200_is_reported_as_changed() {
        let t = FakeTransport::with(vec![ok("a", None), ok("b", None), ok("b", None)]);
        let clock = FakeClock::default();
        let mut f = fetcher(&t, &clock, quick());
        f.fetch(URL).unwrap();
        assert!(matches!(
            f.fetch(URL).unwrap(),
            FetchOutcome::Fetched { changed: true, .. }
        ));
        assert!(matches!(
            f.fetch(URL).unwrap(),
            FetchOutcome::Fetched { changed: false, .. }
        ));
    }

    #[test]
    fn retries_with_exponential_backoff_capped_then_succeeds() {
        let t = FakeTransport::with(vec![
            status(503),
            Err(TransportError::Timeout),
            status(429),
            ok("ok", None),
        ]);
        let clock = FakeClock::default();
        let mut f = fetcher(
            &t,
            &clock,
            FetchPolicy {
                backoff_base_ms: 500,
                backoff_cap_ms: 1_000,
                ..quick()
            },
        );
        assert!(matches!(
            f.fetch(URL).unwrap(),
            FetchOutcome::Fetched { .. }
        ));
        assert_eq!(f.metrics.retries, 3);
        assert_eq!(f.metrics.requests_executed, 4);
        let total: u64 = clock.slept.borrow().iter().sum();
        assert_eq!(total, 500 + 1_000 + 1_000);
    }

    #[test]
    fn retries_are_bounded_and_failure_is_counted_once() {
        let t = FakeTransport::with(vec![status(503), status(503), status(503), status(503)]);
        let clock = FakeClock::default();
        let mut f = fetcher(&t, &clock, quick());
        assert_eq!(f.fetch(URL), Err(FetchError::Http(503)));
        assert_eq!(
            (
                f.metrics.requests_executed,
                f.metrics.retries,
                f.metrics.failures
            ),
            (4, 3, 1)
        );
    }

    #[test]
    fn timeout_is_explicit_in_request_and_reported() {
        let t = FakeTransport::with(vec![Err(TransportError::Timeout); 4]);
        let clock = FakeClock::default();
        let mut f = fetcher(&t, &clock, quick());
        assert_eq!(f.fetch(URL), Err(FetchError::Timeout));
        assert!(t.seen.borrow().iter().all(|s| s.2 == 45_000));
    }

    #[test]
    fn network_failure_is_reported() {
        let t = FakeTransport::with(vec![Err(TransportError::Network("dns".into())); 4]);
        let clock = FakeClock::default();
        let mut f = fetcher(&t, &clock, quick());
        assert_eq!(f.fetch(URL), Err(FetchError::Network("dns".into())));
    }

    #[test]
    fn session_expiry_is_not_retried_and_never_renewed() {
        for script in [
            status(401),
            status(403),
            Ok(Response {
                status: 200,
                final_url: Some("https://sis.lyceum.com.br/login".into()),
                ..Default::default()
            }),
        ] {
            let t = FakeTransport::with(vec![script]);
            let clock = FakeClock::default();
            let mut f = fetcher(&t, &clock, quick());
            assert_eq!(f.fetch(URL), Err(FetchError::SessionExpired));
            assert_eq!(t.seen.borrow().len(), 1);
        }
    }

    #[test]
    fn cancellation_during_backoff_stops_immediately() {
        let t = FakeTransport::with(vec![status(503), ok("x", None)]);
        let clock = FakeClock::default();
        let cancel = CancelToken::new();
        *clock.cancel_after_sleep.borrow_mut() = Some(cancel.clone());
        let mut f = Fetcher::new(
            &t,
            clock.clone(),
            quick(),
            ReadOnlyPolicy::uva_default(),
            cancel,
        )
        .unwrap();
        assert_eq!(f.fetch(URL), Err(FetchError::Cancelled));
        assert_eq!(t.seen.borrow().len(), 1);
        assert!(clock.slept.borrow().iter().all(|s| *s <= 100));
        assert_eq!(f.metrics.failures, 0);
    }

    #[test]
    fn cancelled_before_start_sends_nothing() {
        let t = FakeTransport::with(vec![]);
        let clock = FakeClock::default();
        let cancel = CancelToken::new();
        cancel.cancel();
        let mut f =
            Fetcher::new(&t, clock, quick(), ReadOnlyPolicy::uva_default(), cancel).unwrap();
        assert_eq!(f.fetch(URL), Err(FetchError::Cancelled));
        assert!(t.seen.borrow().is_empty());
    }

    #[test]
    fn request_and_byte_budgets_are_enforced() {
        let t = FakeTransport::with(vec![ok("a", None), ok("b", None)]);
        let clock = FakeClock::default();
        let mut f = fetcher(
            &t,
            &clock,
            FetchPolicy {
                max_requests: 1,
                ..quick()
            },
        );
        f.fetch(URL).unwrap();
        assert_eq!(
            f.fetch("https://uva.instructure.com/api/v1/other"),
            Err(FetchError::LimitExceeded("número de requisições"))
        );

        let t = FakeTransport::with(vec![ok("0123456789", None)]);
        let mut f = fetcher(
            &t,
            &clock,
            FetchPolicy {
                max_body_bytes: 4,
                ..quick()
            },
        );
        assert_eq!(
            f.fetch(URL),
            Err(FetchError::LimitExceeded("tamanho da resposta"))
        );
    }

    #[test]
    fn duration_budget_is_enforced() {
        let clock = FakeClock::default();
        let t = FakeTransport {
            script: RefCell::new(vec![ok("a", None)].into()),
            clock: Some(clock.clone()),
            advance_per_call: 5_000,
            ..Default::default()
        };
        let mut f = fetcher(
            &t,
            &clock,
            FetchPolicy {
                max_duration_ms: 4_000,
                ..quick()
            },
        );
        f.fetch(URL).unwrap();
        assert_eq!(
            f.fetch("https://uva.instructure.com/api/v1/other"),
            Err(FetchError::LimitExceeded("duração máxima"))
        );
    }

    #[test]
    fn rate_limit_spaces_requests() {
        let t = FakeTransport::with(vec![ok("a", None), ok("b", None)]);
        let clock = FakeClock::default();
        let mut f = fetcher(
            &t,
            &clock,
            FetchPolicy {
                min_interval_ms: 250,
                fresh_ttl_ms: 0,
                ..Default::default()
            },
        );
        f.fetch(URL).unwrap();
        f.fetch("https://uva.instructure.com/api/v1/other").unwrap();
        assert_eq!(clock.slept.borrow().iter().sum::<u64>(), 250);
    }

    #[test]
    fn mutating_or_foreign_urls_never_reach_the_transport() {
        let t = FakeTransport::with(vec![]);
        let clock = FakeClock::default();
        let mut f = fetcher(&t, &clock, quick());
        for url in [
            "https://evil.example/x",
            "http://uva.instructure.com/x",
            "https://sis.lyceum.com.br/login",
        ] {
            assert!(matches!(f.fetch(url), Err(FetchError::Blocked(_))), "{url}");
        }
        assert!(t.seen.borrow().is_empty());
    }

    #[test]
    fn unexpected_304_without_cache_is_an_error() {
        let t = FakeTransport::with(vec![status(304)]);
        let clock = FakeClock::default();
        let mut f = fetcher(&t, &clock, quick());
        assert_eq!(f.fetch(URL), Err(FetchError::UnexpectedNotModified));
    }

    #[test]
    fn invalid_policy_is_rejected() {
        let t = FakeTransport::with(vec![]);
        for p in [
            FetchPolicy {
                max_retries: 6,
                ..Default::default()
            },
            FetchPolicy {
                timeout_ms: 10,
                ..Default::default()
            },
            FetchPolicy {
                max_requests: 0,
                ..Default::default()
            },
        ] {
            assert!(
                Fetcher::new(
                    &t,
                    FakeClock::default(),
                    p,
                    ReadOnlyPolicy::uva_default(),
                    CancelToken::new()
                )
                .is_err()
            );
        }
    }

    #[test]
    fn chromium_traffic_is_reported_as_not_measured_until_recorded() {
        let mut m = SyncMetrics::default();
        assert!(m.report_lines().last().unwrap().contains("não medido"));
        m.record_chromium_traffic(Counters {
            requests: 12,
            bytes: 3_400,
        });
        assert!(m.report_lines().last().unwrap().contains("12 requisições"));
    }

    #[test]
    fn pagination_stops_at_page_and_record_limits_and_marks_partial() {
        let cancel = CancelToken::new();
        let pages = |n: u32| -> Result<(Vec<u32>, bool), ()> {
            Ok(((0..3).map(|i| n * 10 + i).collect(), true))
        };
        let r = collect_pages(
            PageLimits {
                max_pages: 2,
                max_records: 100,
            },
            &cancel,
            pages,
        )
        .unwrap();
        assert_eq!((r.items.len(), r.pages, r.complete), (6, 2, false));
        let r = collect_pages(
            PageLimits {
                max_pages: 10,
                max_records: 4,
            },
            &cancel,
            pages,
        )
        .unwrap();
        assert_eq!((r.items.len(), r.complete), (4, false));
        let r = collect_pages(
            PageLimits {
                max_pages: 10,
                max_records: 100,
            },
            &cancel,
            |n| -> Result<_, ()> { Ok((vec![n], n < 3)) },
        )
        .unwrap();
        assert_eq!((r.items, r.complete, r.pages), (vec![1, 2, 3], true, 3));
    }

    #[test]
    fn pagination_propagates_errors_and_cancellation() {
        let cancel = CancelToken::new();
        let limits = PageLimits {
            max_pages: 5,
            max_records: 10,
        };
        assert_eq!(
            collect_pages::<u32, _>(limits, &cancel, |_| Err("falhou")),
            Err(PageError::Source("falhou"))
        );
        cancel.cancel();
        assert_eq!(
            collect_pages::<u32, ()>(limits, &cancel, |_| Ok((vec![], false))),
            Err(PageError::Cancelled)
        );
    }
}
