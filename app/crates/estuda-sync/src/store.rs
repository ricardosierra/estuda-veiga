//! Estado em RAM: cache limitado com TTL e invalidação por hash, caches privado e público
//! em tipos distintos, e deduplicação por identificador de fonte e por hash de conteúdo.
//!
//! O SQLite é só persistência; consultas comuns são atendidas daqui (cache válido) e o disco
//! é consultado apenas em *miss*. Este módulo não conhece banco.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::privacy::PublicRecord;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub invalidations: u64,
}

#[derive(Debug)]
struct Entry<V> {
    value: V,
    content_hash: String,
    stored_at_ms: u64,
    last_used: u64,
}

#[derive(Debug)]
pub struct BoundedCache<K, V> {
    capacity: usize,
    ttl_ms: u64,
    entries: HashMap<K, Entry<V>>,
    tick: u64,
    pub stats: CacheStats,
}

impl<K: Hash + Eq + Clone, V> BoundedCache<K, V> {
    pub fn new(capacity: usize, ttl_ms: u64) -> Self {
        Self {
            capacity: capacity.max(1),
            ttl_ms,
            entries: HashMap::new(),
            tick: 0,
            stats: CacheStats::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// `expected_hash`: hash atual da fonte (ex.: ETag/sha256). Diferente ⇒ entrada invalidada.
    pub fn get(&mut self, key: &K, now_ms: u64, expected_hash: Option<&str>) -> Option<&V> {
        self.tick += 1;
        let tick = self.tick;
        let stale = match self.entries.get(key) {
            None => {
                self.stats.misses += 1;
                return None;
            }
            Some(e) => {
                now_ms.saturating_sub(e.stored_at_ms) >= self.ttl_ms
                    || expected_hash.is_some_and(|h| h != e.content_hash)
            }
        };
        if stale {
            self.entries.remove(key);
            self.stats.invalidations += 1;
            self.stats.misses += 1;
            return None;
        }
        self.stats.hits += 1;
        let entry = self.entries.get_mut(key)?;
        entry.last_used = tick;
        Some(&entry.value)
    }

    pub fn insert(&mut self, key: K, value: V, content_hash: &str, now_ms: u64) {
        self.tick += 1;
        if !self.entries.contains_key(&key)
            && self.entries.len() >= self.capacity
            && let Some(oldest) = self
                .entries
                .iter()
                .min_by_key(|(_, e)| e.last_used)
                .map(|(k, _)| k.clone())
        {
            self.entries.remove(&oldest);
            self.stats.evictions += 1;
        }
        self.entries.insert(
            key,
            Entry {
                value,
                content_hash: content_hash.to_owned(),
                stored_at_ms: now_ms,
                last_used: self.tick,
            },
        );
    }

    pub fn invalidate(&mut self, key: &K) -> bool {
        let removed = self.entries.remove(key).is_some();
        if removed {
            self.stats.invalidations += 1;
        }
        removed
    }
}

/// Cache compartilhável entre threads. O carregador roda **fora** do lock.
pub struct SharedCache<K, V> {
    inner: Arc<Mutex<BoundedCache<K, V>>>,
}

impl<K, V> Clone for SharedCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K: Hash + Eq + Clone, V: Clone> SharedCache<K, V> {
    pub fn new(capacity: usize, ttl_ms: u64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(BoundedCache::new(capacity, ttl_ms))),
        }
    }

    fn lock(&self) -> MutexGuard<'_, BoundedCache<K, V>> {
        // Um pânico em outra thread não deve derrubar o cache: os dados continuam consistentes.
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn get(&self, key: &K, now_ms: u64, expected_hash: Option<&str>) -> Option<V> {
        self.lock().get(key, now_ms, expected_hash).cloned()
    }

    pub fn insert(&self, key: K, value: V, content_hash: &str, now_ms: u64) {
        self.lock().insert(key, value, content_hash, now_ms);
    }

    pub fn invalidate(&self, key: &K) -> bool {
        self.lock().invalidate(key)
    }

    pub fn stats(&self) -> CacheStats {
        self.lock().stats
    }

    /// Cache válido ⇒ nenhum acesso ao disco; *miss* ⇒ `loader` (disco/rede) uma vez.
    pub fn get_or_load(
        &self,
        key: &K,
        now_ms: u64,
        expected_hash: Option<&str>,
        loader: impl FnOnce() -> Option<(V, String)>,
    ) -> Option<V> {
        if let Some(v) = self.get(key, now_ms, expected_hash) {
            return Some(v);
        }
        let (value, hash) = loader()?;
        self.insert(key.clone(), value.clone(), &hash, now_ms);
        Some(value)
    }
}

/// Cache do domínio privado: notas, frequência, matrícula, entregas.
pub struct PrivateCache<V> {
    inner: SharedCache<String, V>,
}

impl<V: Clone> PrivateCache<V> {
    pub fn new(capacity: usize, ttl_ms: u64) -> Self {
        Self {
            inner: SharedCache::new(capacity, ttl_ms),
        }
    }
    pub fn cache(&self) -> &SharedCache<String, V> {
        &self.inner
    }
}

/// Cache do catálogo público. Só aceita [`PublicRecord`], que só o pipeline de
/// [`crate::privacy`] consegue produzir.
pub struct PublicCache {
    inner: SharedCache<String, PublicRecord>,
}

impl PublicCache {
    pub fn new(capacity: usize, ttl_ms: u64) -> Self {
        Self {
            inner: SharedCache::new(capacity, ttl_ms),
        }
    }
    pub fn insert(&self, record: PublicRecord, now_ms: u64) {
        let hash = record.content_hash().to_owned();
        self.inner
            .insert(record.id().to_owned(), record, &hash, now_ms);
    }
    pub fn get(&self, id: &str, now_ms: u64, expected_hash: Option<&str>) -> Option<PublicRecord> {
        self.inner.get(&id.to_owned(), now_ms, expected_hash)
    }
}

// ---------------------------------------------------------------------------------------
// Deduplicação
// ---------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dedup {
    New,
    Unchanged,
    Updated {
        previous_hash: String,
    },
    /// Mesmo conteúdo já visto sob outro identificador de fonte (ex.: mesmo PDF em dois cursos).
    DuplicateContent {
        of: String,
    },
}

#[derive(Debug, Default)]
pub struct DedupIndex {
    by_source: HashMap<String, String>,
    first_by_hash: HashMap<String, String>,
}

impl DedupIndex {
    /// `source_key` é estável (`canvas:course:123:item:45`), nunca derivado só do título.
    pub fn register(&mut self, source_key: &str, content_hash: &str) -> Dedup {
        match self.by_source.get(source_key) {
            Some(prev) if prev == content_hash => return Dedup::Unchanged,
            Some(prev) => {
                let previous_hash = prev.clone();
                self.by_source
                    .insert(source_key.to_owned(), content_hash.to_owned());
                self.first_by_hash
                    .entry(content_hash.to_owned())
                    .or_insert_with(|| source_key.to_owned());
                return Dedup::Updated { previous_hash };
            }
            None => {}
        }
        self.by_source
            .insert(source_key.to_owned(), content_hash.to_owned());
        match self.first_by_hash.get(content_hash) {
            Some(first) => Dedup::DuplicateContent { of: first.clone() },
            None => {
                self.first_by_hash
                    .insert(content_hash.to_owned(), source_key.to_owned());
                Dedup::New
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::privacy::{Candidate, ContentOrigin, Discovered, License};

    #[test]
    fn valid_cache_hits_without_calling_the_loader() {
        let c: SharedCache<String, u32> = SharedCache::new(4, 1_000);
        let mut loads = 0;
        for _ in 0..3 {
            let v = c.get_or_load(&"a".to_owned(), 10, None, || {
                loads += 1;
                Some((7, "h1".into()))
            });
            assert_eq!(v, Some(7));
        }
        assert_eq!(loads, 1);
        assert_eq!((c.stats().hits, c.stats().misses), (2, 1));
    }

    #[test]
    fn expired_entry_is_reloaded() {
        let c: SharedCache<String, u32> = SharedCache::new(4, 1_000);
        c.insert("a".into(), 1, "h", 0);
        assert_eq!(c.get(&"a".to_owned(), 999, None), Some(1));
        assert_eq!(c.get(&"a".to_owned(), 1_000, None), None);
        assert_eq!(c.stats().invalidations, 1);
    }

    #[test]
    fn hash_change_invalidates() {
        let c: SharedCache<String, u32> = SharedCache::new(4, 10_000);
        c.insert("a".into(), 1, "h1", 0);
        assert_eq!(c.get(&"a".to_owned(), 1, Some("h1")), Some(1));
        assert_eq!(c.get(&"a".to_owned(), 2, Some("h2")), None);
        assert_eq!(c.get(&"a".to_owned(), 3, None), None);
    }

    #[test]
    fn least_recently_used_is_evicted_at_capacity() {
        let c: SharedCache<String, u32> = SharedCache::new(2, 10_000);
        c.insert("a".into(), 1, "h", 0);
        c.insert("b".into(), 2, "h", 0);
        assert_eq!(c.get(&"a".to_owned(), 1, None), Some(1)); // a fica recente
        c.insert("c".into(), 3, "h", 2);
        assert_eq!(c.get(&"b".to_owned(), 3, None), None);
        assert_eq!(c.get(&"a".to_owned(), 3, None), Some(1));
        assert_eq!(c.stats().evictions, 1);
    }

    #[test]
    fn loader_failure_is_not_cached() {
        let c: SharedCache<String, u32> = SharedCache::new(2, 10_000);
        assert_eq!(c.get_or_load(&"a".to_owned(), 0, None, || None), None);
        assert_eq!(
            c.get_or_load(&"a".to_owned(), 0, None, || Some((9, "h".into()))),
            Some(9)
        );
    }

    #[test]
    fn concurrent_access_is_safe() {
        let c: SharedCache<String, u64> = SharedCache::new(64, 1_000_000);
        let handles: Vec<_> = (0..8u64)
            .map(|t| {
                let c = c.clone();
                std::thread::spawn(move || {
                    for i in 0..200u64 {
                        let key = format!("k{}", i % 32);
                        c.get_or_load(&key, i, None, || Some((t * 1000 + i, format!("h{i}"))));
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        let s = c.stats();
        assert_eq!(s.hits + s.misses, 8 * 200); // cada consulta conta exatamente uma vez
        assert!(s.hits > 0);
    }

    fn public_record(id: &str) -> PublicRecord {
        Discovered::new(Candidate {
            id: id.into(),
            title: "Aula".into(),
            origin: ContentOrigin::OfficialPublication,
            license: License::Verified { note: "ok".into() },
            source_url: Some("https://ead.uva.br/a".into()),
            text: "conteúdo".into(),
            declared_hash: None,
        })
        .verify_origin()
        .unwrap()
        .remove_personal_data()
        .unwrap()
        .check_integrity()
        .unwrap()
        .approve("revisor")
        .unwrap()
    }

    #[test]
    fn public_and_private_caches_are_physically_separate() {
        let public = PublicCache::new(8, 1_000);
        let private: PrivateCache<String> = PrivateCache::new(8, 1_000);
        public.insert(public_record("mat-1"), 0);
        private
            .cache()
            .insert("nota:algoritmos".into(), "8,5".into(), "h", 0);
        assert!(public.get("mat-1", 1, None).is_some());
        assert!(public.get("nota:algoritmos", 1, None).is_none());
        assert!(private.cache().get(&"mat-1".to_owned(), 1, None).is_none());
    }

    #[test]
    fn dedup_by_source_and_by_content() {
        let mut d = DedupIndex::default();
        assert_eq!(d.register("canvas:1:item:1", "h1"), Dedup::New);
        assert_eq!(d.register("canvas:1:item:1", "h1"), Dedup::Unchanged);
        assert_eq!(
            d.register("canvas:2:item:9", "h1"),
            Dedup::DuplicateContent {
                of: "canvas:1:item:1".into()
            }
        );
        assert_eq!(
            d.register("canvas:1:item:1", "h2"),
            Dedup::Updated {
                previous_hash: "h1".into()
            }
        );
    }
}
