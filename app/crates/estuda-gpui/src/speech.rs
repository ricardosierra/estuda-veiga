//! Narração em voz (texto para áudio) com o eSpeak NG, o motor que o Ubuntu
//! usa por padrão (via Speech Dispatcher/Orca).
//!
//! - Motor: eSpeak NG (GPL-3.0-or-later), compilado dentro do binário pelo
//!   crate `espeak-rs-sys`. Só os dados de voz (`espeak-ng-data`) ficam em disco.
//! - Dados: ver [`localizar_dados`].
//! - Reprodução: rodio 0.23 numa thread de fundo. A síntese é feita em trechos
//!   curtos, para que o início seja rápido e a parada seja imediata.

#![allow(dead_code)] // a interface ainda não é consumida pela UI

use std::ffi::{CString, c_int, c_void};
use std::fmt;
use std::num::NonZero;
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use espeak_rs_sys as es;
use rodio::buffer::SamplesBuffer;
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};

/// Voz pedida ao eSpeak NG: português do Brasil.
const VOZ_PT_BR: &str = "pt-br";
/// Nome da pasta de dados do eSpeak NG.
const PASTA_DADOS: &str = "espeak-ng-data";
/// Variável de ambiente com o diretório que contém `espeak-ng-data`.
const VARIAVEL_DADOS: &str = "ESTUDA_ESPEAK_DATA";
/// Tamanho máximo, em bytes, de cada trecho sintetizado de uma vez.
const TAMANHO_TRECHO: usize = 180;
/// Quantos trechos podem esperar na fila de reprodução antes de sintetizar mais.
const TRECHOS_NA_FILA: usize = 2;
/// Intervalo em que a reprodução verifica comandos novos.
const INTERVALO: Duration = Duration::from_millis(15);

/// Erros da narração.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeechError {
    /// O eSpeak NG não iniciou ou não conseguiu sintetizar (dados ausentes ou falha interna).
    Motor(String),
    /// Não há dispositivo de saída de áudio utilizável.
    Audio(String),
}

impl fmt::Display for SpeechError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpeechError::Motor(msg) => write!(f, "falha no motor de voz eSpeak-ng: {msg}"),
            SpeechError::Audio(msg) => write!(f, "falha na saída de áudio: {msg}"),
        }
    }
}

impl std::error::Error for SpeechError {}

/// Estado do eSpeak NG. O motor é um singleton do processo.
struct Motor {
    taxa: NonZero<u32>,
    voz_pt_br: bool,
    aviso: Option<String>,
}

static MOTOR: OnceLock<Result<Motor, SpeechError>> = OnceLock::new();
/// Serializa toda chamada ao eSpeak NG, que não é thread-safe.
static TRAVA_MOTOR: Mutex<()> = Mutex::new(());
/// Síntese em andamento. O callback C de amostras escreve aqui.
static CAPTURA: Mutex<Option<Captura>> = Mutex::new(None);

struct Captura {
    amostras: Vec<i16>,
    cancelar: Arc<AtomicBool>,
}

fn trava<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Callback chamado pelo eSpeak NG com cada bloco de amostras PCM 16 bits mono.
/// Retorna 1 para abortar a síntese, 0 para continuar.
unsafe extern "C" fn ao_receber_amostras(
    wav: *mut i16,
    quantidade: c_int,
    _eventos: *mut es::espeak_EVENT,
) -> c_int {
    let mut guarda = trava(&CAPTURA);
    let Some(captura) = guarda.as_mut() else {
        return 1;
    };
    if captura.cancelar.load(Ordering::SeqCst) {
        return 1;
    }
    if !wav.is_null() && quantidade > 0 {
        // SAFETY: o eSpeak NG entrega `quantidade` amostras válidas em `wav`.
        let fatia = unsafe { std::slice::from_raw_parts(wav, quantidade as usize) };
        captura.amostras.extend_from_slice(fatia);
    }
    0
}

/// Localiza o diretório que contém a pasta `espeak-ng-data`.
///
/// Ordem de busca:
/// 1. variável `ESTUDA_ESPEAK_DATA`;
/// 2. ao lado do binário (forma de distribuição: copiar `espeak-ng-data` junto);
/// 3. cache de build do cargo, em `target/<perfil>/build/espeak-rs-sys-*/out/share`
///    (desenvolvimento: o `build.rs` do `espeak-rs-sys` gera os dados ali e não os
///    copia para `target/`). Também é procurado a partir de `target/<perfil>/deps`,
///    onde ficam os binários de teste;
/// 4. diretório atual.
///
/// Se nada for encontrado, o motor usa o caminho que ele tem embutido no build.
pub fn localizar_dados() -> Option<PathBuf> {
    if let Some(valor) = std::env::var_os(VARIAVEL_DADOS) {
        let dir = PathBuf::from(valor);
        if dir.join(PASTA_DADOS).is_dir() {
            return Some(dir);
        }
    }
    let dir_exe = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf));
    if let Some(dir_exe) = dir_exe {
        let bases = [Some(dir_exe.clone()), dir_exe.parent().map(Path::to_path_buf)];
        for base in bases.into_iter().flatten() {
            if base.join(PASTA_DADOS).is_dir() {
                return Some(base);
            }
            if let Some(share) = share_do_cache_de_build(&base) {
                return Some(share);
            }
        }
    }
    let cwd = std::env::current_dir().ok()?;
    cwd.join(PASTA_DADOS).is_dir().then_some(cwd)
}

/// Diretório `out/share` do build mais recente do `espeak-rs-sys` que já gerou os dados.
fn share_do_cache_de_build(perfil: &Path) -> Option<PathBuf> {
    let mut melhor: Option<(SystemTime, PathBuf)> = None;
    for entrada in std::fs::read_dir(perfil.join("build")).ok()?.flatten() {
        if !entrada
            .file_name()
            .to_string_lossy()
            .starts_with("espeak-rs-sys-")
        {
            continue;
        }
        let share = entrada.path().join("out").join("share");
        if !share.join(PASTA_DADOS).is_dir() {
            continue;
        }
        let momento = std::fs::metadata(&share)
            .and_then(|m| m.modified())
            .unwrap_or(UNIX_EPOCH);
        if melhor.as_ref().is_none_or(|(atual, _)| momento > *atual) {
            melhor = Some((momento, share));
        }
    }
    melhor.map(|(_, share)| share)
}

fn motor() -> Result<&'static Motor, SpeechError> {
    MOTOR
        .get_or_init(iniciar_motor)
        .as_ref()
        .map_err(Clone::clone)
}

fn iniciar_motor() -> Result<Motor, SpeechError> {
    let dados = localizar_dados();
    let caminho = match dados.as_ref() {
        Some(dir) => Some(
            CString::new(dir.to_string_lossy().into_owned())
                .map_err(|_| SpeechError::Motor("caminho dos dados contém byte nulo".into()))?,
        ),
        None => None,
    };
    let ptr_caminho = caminho.as_ref().map_or(ptr::null(), |c| c.as_ptr());

    let _trava = trava(&TRAVA_MOTOR);
    // SAFETY: caminho é uma string C válida durante a chamada, ou nulo (caminho embutido).
    let taxa = unsafe {
        es::espeak_Initialize(
            es::espeak_AUDIO_OUTPUT_AUDIO_OUTPUT_RETRIEVAL,
            0,
            ptr_caminho,
            es::espeakINITIALIZE_DONT_EXIT as c_int,
        )
    };
    let Some(taxa) = u32::try_from(taxa).ok().and_then(NonZero::<u32>::new) else {
        let onde = match dados {
            Some(dir) => format!("dados lidos de `{}`", dir.display()),
            None => "nenhuma pasta de dados encontrada".to_string(),
        };
        return Err(SpeechError::Motor(format!(
            "não foi possível carregar a voz (código {taxa}; {onde}). \
             Coloque a pasta `{PASTA_DADOS}` ao lado do binário ou defina `{VARIAVEL_DADOS}`."
        )));
    };

    // SAFETY: registra um callback estático, chamado pelo motor durante a síntese.
    unsafe { es::espeak_SetSynthCallback(Some(ao_receber_amostras)) };

    let nome = CString::new(VOZ_PT_BR).expect("nome fixo sem byte nulo");
    // SAFETY: `nome` é uma string C válida durante a chamada.
    let voz_pt_br = unsafe { es::espeak_SetVoiceByName(nome.as_ptr()) } == es::espeak_ERROR_EE_OK;
    let aviso = (!voz_pt_br).then(|| {
        format!(
            "voz `{VOZ_PT_BR}` ausente no eSpeak-ng embutido; usando a voz padrão do motor \
             (a fala pode soar em inglês)"
        )
    });
    Ok(Motor {
        taxa,
        voz_pt_br,
        aviso,
    })
}

/// Amostras PCM 16 bits mono de uma síntese e a taxa de amostragem.
#[derive(Debug, Clone, PartialEq)]
pub struct Sintese {
    pub taxa: u32,
    pub amostras: Vec<i16>,
}

/// Sintetiza `texto` em português do Brasil, sem reproduzir.
pub fn sintetizar(texto: &str) -> Result<Sintese, SpeechError> {
    let motor = motor()?;
    let amostras = sintetizar_trecho(motor, texto, &Arc::new(AtomicBool::new(false)))?;
    Ok(Sintese {
        taxa: motor.taxa.get(),
        amostras,
    })
}

/// Sintetiza um trecho. Retorna antes, ou com amostras parciais, se `cancelar` for ligado.
fn sintetizar_trecho(
    motor: &Motor,
    texto: &str,
    cancelar: &Arc<AtomicBool>,
) -> Result<Vec<i16>, SpeechError> {
    let limpo: String = texto.chars().filter(|&c| c != '\0').collect();
    if limpo.trim().is_empty() || cancelar.load(Ordering::SeqCst) {
        return Ok(Vec::new());
    }
    let bytes = CString::new(limpo).map_err(|_| SpeechError::Motor("texto com byte nulo".into()))?;

    let _trava = trava(&TRAVA_MOTOR);
    if motor.voz_pt_br {
        let nome = CString::new(VOZ_PT_BR).expect("nome fixo sem byte nulo");
        // SAFETY: `nome` é uma string C válida durante a chamada.
        unsafe { es::espeak_SetVoiceByName(nome.as_ptr()) };
    }
    *trava(&CAPTURA) = Some(Captura {
        amostras: Vec::new(),
        cancelar: Arc::clone(cancelar),
    });
    let flags: u32 = es::espeakCHARS_UTF8 as u32 | es::espeakENDPAUSE as u32;
    // SAFETY: `bytes` vive até o fim da chamada; o motor copia o texto antes de retornar.
    let status = unsafe {
        es::espeak_Synth(
            bytes.as_ptr().cast::<c_void>(),
            bytes.as_bytes().len(),
            0,
            es::espeak_POSITION_TYPE_POS_CHARACTER,
            0,
            flags,
            ptr::null_mut(),
            ptr::null_mut(),
        )
    };
    // SAFETY: espera o fim do trabalho assíncrono do motor antes de ler a captura.
    unsafe { es::espeak_Synchronize() };
    let captura = trava(&CAPTURA).take();
    if cancelar.load(Ordering::SeqCst) {
        // SAFETY: descarta o que restou na fila interna do motor.
        unsafe { es::espeak_Cancel() };
    }
    if status != es::espeak_ERROR_EE_OK {
        return Err(SpeechError::Motor(format!("espeak_Synth retornou {status}")));
    }
    Ok(captura.map(|c| c.amostras).unwrap_or_default())
}

/// Converte Markdown em texto corrido para leitura em voz alta.
///
/// Remove cabeçalhos, marcadores de lista, citações, caixas de tarefa, negrito,
/// itálico, `#`, `|` de tabelas e separadores horizontais. Código inline perde
/// as crases e mantém o texto. Links e imagens viram só o rótulo. Blocos de
/// código cercados são pulados.
pub fn plain_text(markdown: &str) -> String {
    let mut linhas = Vec::new();
    let mut em_bloco_de_codigo = false;
    for linha in markdown.lines() {
        let aparada = linha.trim();
        if aparada.starts_with("```") || aparada.starts_with("~~~") {
            em_bloco_de_codigo = !em_bloco_de_codigo;
            continue;
        }
        if em_bloco_de_codigo || e_separador(aparada) {
            continue;
        }
        let texto = limpar_inline(remover_marcadores_de_bloco(aparada));
        let normalizado = texto.split_whitespace().collect::<Vec<_>>().join(" ");
        if !normalizado.is_empty() {
            linhas.push(normalizado);
        }
    }
    linhas.join("\n")
}

/// Remove prefixos de bloco: cabeçalhos, citações, listas e caixas de tarefa.
fn remover_marcadores_de_bloco(linha: &str) -> &str {
    let mut resto = linha;
    loop {
        let anterior = resto;
        if resto.starts_with('#') {
            let sem_cerquilhas = resto.trim_start_matches('#');
            if sem_cerquilhas.is_empty() || sem_cerquilhas.starts_with(' ') {
                resto = sem_cerquilhas.trim_start();
            }
        }
        if let Some(r) = resto.strip_prefix('>') {
            resto = r.trim_start();
        }
        if let Some(r) = ["- ", "* ", "+ "].into_iter().find_map(|m| resto.strip_prefix(m)) {
            resto = r.trim_start();
        }
        let digitos = resto.bytes().take_while(u8::is_ascii_digit).count();
        if digitos > 0 {
            let depois = &resto[digitos..];
            if let Some(r) = depois
                .strip_prefix(". ")
                .or_else(|| depois.strip_prefix(") "))
            {
                resto = r.trim_start();
            }
        }
        if let Some(r) = ["[ ] ", "[x] ", "[X] "]
            .into_iter()
            .find_map(|m| resto.strip_prefix(m))
        {
            resto = r;
        }
        if resto == anterior {
            return resto;
        }
    }
}

/// Linhas que são só regra horizontal (`---`, `***`) ou separador de tabela (`|---|:--|`).
fn e_separador(linha: &str) -> bool {
    let marcas = linha
        .chars()
        .filter(|c| matches!(c, '-' | '*' | '_'))
        .count();
    marcas >= 3
        && linha
            .chars()
            .all(|c| matches!(c, '-' | '*' | '_' | '|' | ':' | ' '))
}

/// Posição de `alvo` a partir de `de`, em um vetor de caracteres.
fn posicao(chars: &[char], de: usize, alvo: char) -> Option<usize> {
    chars[de..]
        .iter()
        .position(|&c| c == alvo)
        .map(|p| p + de)
}

/// Limpa marcadores inline de um trecho de Markdown.
fn limpar_inline(texto: &str) -> String {
    let c: Vec<char> = texto.chars().collect();
    let mut saida = String::with_capacity(texto.len());
    let mut i = 0;
    while i < c.len() {
        let atual = c[i];

        // Link [rótulo](url) e imagem ![alt](url): fica só o rótulo.
        let abre = if atual == '!' && c.get(i + 1) == Some(&'[') {
            Some(i + 1)
        } else if atual == '[' {
            Some(i)
        } else {
            None
        };
        if let Some(abre) = abre
            && let Some(fecha) = posicao(&c, abre, ']')
            && c.get(fecha + 1) == Some(&'(')
            && let Some(fim) = posicao(&c, fecha + 1, ')')
        {
            saida.extend(&c[abre + 1..fecha]);
            i = fim + 1;
            continue;
        }

        // Autolink <https://...> ou <autor@dominio>: a URL não é lida.
        if atual == '<'
            && let Some(fim) = posicao(&c, i, '>')
        {
            let interno: String = c[i + 1..fim].iter().collect();
            if interno.contains("://") || interno.contains('@') {
                i = fim + 1;
                continue;
            }
        }

        match atual {
            // Entre letras (snake_case) vira espaço; em outros casos some.
            '*' | '_' | '~' => {
                let entre_letras = i > 0
                    && c[i - 1].is_alphanumeric()
                    && c.get(i + 1).is_some_and(|p| p.is_alphanumeric());
                if entre_letras {
                    saida.push(' ');
                }
            }
            '|' | '[' | ']' => saida.push(' '),
            '`' | '#' => {}
            outro => saida.push(outro),
        }
        i += 1;
    }
    saida
}

/// Divide o texto em trechos curtos, em fronteiras de frase.
fn dividir_em_trechos(texto: &str) -> Vec<String> {
    let mut trechos = Vec::new();
    let mut atual = String::new();
    for linha in texto.lines() {
        for frase in dividir_frases(linha) {
            for pedaco in quebrar_frase(&frase) {
                if !atual.is_empty() && atual.len() + 1 + pedaco.len() > TAMANHO_TRECHO {
                    trechos.push(std::mem::take(&mut atual));
                }
                if !atual.is_empty() {
                    atual.push(' ');
                }
                atual.push_str(&pedaco);
            }
        }
        // Fim de linha encerra o trecho: os títulos e itens ganham pausa natural.
        if !atual.is_empty() {
            trechos.push(std::mem::take(&mut atual));
        }
    }
    trechos
}

fn dividir_frases(linha: &str) -> Vec<String> {
    let mut frases = Vec::new();
    let mut atual = String::new();
    let mut caracteres = linha.chars().peekable();
    while let Some(c) = caracteres.next() {
        atual.push(c);
        let fim_de_frase = matches!(c, '.' | '!' | '?' | ';' | ':')
            && caracteres.peek().is_none_or(|p| p.is_whitespace());
        if fim_de_frase {
            frases.push(std::mem::take(&mut atual));
        }
    }
    frases.push(atual);
    frases
        .into_iter()
        .map(|f| f.trim().to_string())
        .filter(|f| !f.is_empty())
        .collect()
}

/// Quebra uma frase longa em pedaços de até `TAMANHO_TRECHO`, em espaços.
fn quebrar_frase(frase: &str) -> Vec<String> {
    if frase.len() <= TAMANHO_TRECHO {
        return vec![frase.to_string()];
    }
    let mut pedacos = Vec::new();
    let mut atual = String::new();
    for palavra in frase.split_whitespace() {
        if !atual.is_empty() && atual.len() + 1 + palavra.len() > TAMANHO_TRECHO {
            pedacos.push(std::mem::take(&mut atual));
        }
        if !atual.is_empty() {
            atual.push(' ');
        }
        atual.push_str(palavra);
    }
    if !atual.is_empty() {
        pedacos.push(atual);
    }
    pedacos
}

/// Comandos enviados à thread de narração.
enum Comando {
    Falar(String),
    Pausar,
    Continuar,
    Parar,
    Encerrar,
}

/// Narrador: fala texto Markdown em português do Brasil, numa thread de fundo.
pub struct Narrator {
    comandos: Sender<Comando>,
    ativa: Arc<AtomicBool>,
    pausada: Arc<AtomicBool>,
    cancelar: Arc<AtomicBool>,
    aviso: Option<String>,
    trabalhador: Option<JoinHandle<()>>,
}

impl Narrator {
    /// Inicializa o eSpeak NG e abre a saída de áudio padrão.
    ///
    /// Se a voz pt-br não existir no motor, o narrador usa a voz padrão e
    /// guarda o aviso, que pode ser lido com [`Narrator::aviso_voz`].
    pub fn new() -> Result<Narrator, SpeechError> {
        let motor = motor()?;
        let (comandos, recebidos) = mpsc::channel();
        let (pronto_tx, pronto_rx) = mpsc::sync_channel(1);
        let ativa = Arc::new(AtomicBool::new(false));
        let pausada = Arc::new(AtomicBool::new(false));
        let cancelar = Arc::new(AtomicBool::new(false));

        let (a, p, c) = (Arc::clone(&ativa), Arc::clone(&pausada), Arc::clone(&cancelar));
        let trabalhador = std::thread::Builder::new()
            .name("estuda-narracao".into())
            .spawn(move || trabalhar(recebidos, a, p, c, motor, pronto_tx))
            .map_err(|e| {
                SpeechError::Audio(format!("não foi possível criar a thread de narração: {e}"))
            })?;

        match pronto_rx.recv() {
            Ok(Ok(())) => Ok(Narrator {
                comandos,
                ativa,
                pausada,
                cancelar,
                aviso: motor.aviso.clone(),
                trabalhador: Some(trabalhador),
            }),
            Ok(Err(erro)) => {
                let _ = trabalhador.join();
                Err(erro)
            }
            Err(_) => {
                let _ = trabalhador.join();
                Err(SpeechError::Audio(
                    "a thread de áudio encerrou antes de iniciar".into(),
                ))
            }
        }
    }

    /// Limpa o Markdown e começa a falar. Interrompe a fala anterior.
    pub fn speak(&self, markdown: &str) {
        self.ativa.store(true, Ordering::SeqCst);
        self.pausada.store(false, Ordering::SeqCst);
        // Ordem importa: o cancelamento deve estar visível antes do comando chegar.
        self.cancelar.store(true, Ordering::SeqCst);
        let _ = self.comandos.send(Comando::Falar(plain_text(markdown)));
    }

    /// Pausa a fala. Sem efeito se não houver fala em andamento.
    pub fn pause(&self) {
        let _ = self.comandos.send(Comando::Pausar);
    }

    /// Continua a fala pausada.
    pub fn resume(&self) {
        let _ = self.comandos.send(Comando::Continuar);
    }

    /// Para a fala e descarta o áudio já sintetizado.
    pub fn stop(&self) {
        self.ativa.store(false, Ordering::SeqCst);
        self.pausada.store(false, Ordering::SeqCst);
        self.cancelar.store(true, Ordering::SeqCst);
        let _ = self.comandos.send(Comando::Parar);
    }

    /// `true` enquanto há uma fala em andamento, inclusive pausada.
    pub fn is_speaking(&self) -> bool {
        self.ativa.load(Ordering::SeqCst)
    }

    /// `true` quando a fala está pausada.
    pub fn is_paused(&self) -> bool {
        self.pausada.load(Ordering::SeqCst)
    }

    /// Aviso sobre a voz, se a pt-br não estiver disponível.
    pub fn aviso_voz(&self) -> Option<&str> {
        self.aviso.as_deref()
    }
}

impl Drop for Narrator {
    fn drop(&mut self) {
        self.stop();
        let _ = self.comandos.send(Comando::Encerrar);
        if let Some(trabalhador) = self.trabalhador.take() {
            let _ = trabalhador.join();
        }
    }
}

/// Estado da thread de reprodução. Só ela fala com o motor e com o `Player`.
struct Trabalhador {
    recebidos: Receiver<Comando>,
    ativa: Arc<AtomicBool>,
    pausada: Arc<AtomicBool>,
    cancelar: Arc<AtomicBool>,
    motor: &'static Motor,
    saida: MixerDeviceSink,
    player: Player,
}

fn trabalhar(
    recebidos: Receiver<Comando>,
    ativa: Arc<AtomicBool>,
    pausada: Arc<AtomicBool>,
    cancelar: Arc<AtomicBool>,
    motor: &'static Motor,
    pronto: SyncSender<Result<(), SpeechError>>,
) {
    let mut saida = match DeviceSinkBuilder::open_default_sink() {
        Ok(saida) => saida,
        Err(erro) => {
            let _ = pronto.send(Err(SpeechError::Audio(format!(
                "saída de áudio padrão indisponível: {erro}"
            ))));
            return;
        }
    };
    saida.log_on_drop(false);
    let player = Player::connect_new(saida.mixer());
    let _ = pronto.send(Ok(()));
    let mut trabalhador = Trabalhador {
        recebidos,
        ativa,
        pausada,
        cancelar,
        motor,
        saida,
        player,
    };
    trabalhador.laco();
}

impl Trabalhador {
    fn laco(&mut self) {
        let mut pendente: Option<Comando> = None;
        loop {
            let comando = match pendente.take() {
                Some(c) => c,
                None => match self.recebidos.recv() {
                    Ok(c) => c,
                    Err(_) => break,
                },
            };
            pendente = match comando {
                Comando::Falar(texto) => self.falar(&texto),
                Comando::Encerrar => break,
                // Fora de uma fala, pausar/continuar/parar não devolvem comando.
                outro => self.tratar(outro).err().flatten(),
            };
        }
    }

    /// Trata um comando. `Err(None)` = parou; `Err(Some(c))` = `c` precisa ser tratado depois.
    fn tratar(&mut self, comando: Comando) -> Result<(), Option<Comando>> {
        match comando {
            Comando::Pausar => {
                if self.ativa.load(Ordering::SeqCst) {
                    self.player.pause();
                    self.pausada.store(true, Ordering::SeqCst);
                }
                Ok(())
            }
            Comando::Continuar => {
                self.player.play();
                self.pausada.store(false, Ordering::SeqCst);
                Ok(())
            }
            Comando::Parar => {
                self.silenciar();
                Err(None)
            }
            outro => Err(Some(outro)),
        }
    }

    /// Consome comandos já recebidos, esperando até `espera` pelo primeiro, se houver.
    fn atender(&mut self, espera: Option<Duration>) -> Result<(), Option<Comando>> {
        if let Some(espera) = espera {
            match self.recebidos.recv_timeout(espera) {
                Ok(comando) => self.tratar(comando)?,
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => return Err(Some(Comando::Encerrar)),
            }
        }
        while let Ok(comando) = self.recebidos.try_recv() {
            self.tratar(comando)?;
        }
        Ok(())
    }

    /// Descarta o áudio em fila. Soltar o `Player` antigo para a reprodução dele.
    fn silenciar(&mut self) {
        self.player = Player::connect_new(self.saida.mixer());
        self.ativa.store(false, Ordering::SeqCst);
        self.pausada.store(false, Ordering::SeqCst);
    }

    /// Fala um texto já limpo. Devolve o comando que a interrompeu, se houver.
    fn falar(&mut self, texto: &str) -> Option<Comando> {
        self.cancelar.store(false, Ordering::SeqCst);
        self.silenciar();
        self.ativa.store(true, Ordering::SeqCst);
        let fim = self.reproduzir(texto);
        self.ativa.store(false, Ordering::SeqCst);
        fim
    }

    fn reproduzir(&mut self, texto: &str) -> Option<Comando> {
        for trecho in dividir_em_trechos(texto) {
            loop {
                if let Err(fim) = self.atender(None) {
                    return fim;
                }
                if self.player.len() < TRECHOS_NA_FILA {
                    break;
                }
                if let Err(fim) = self.atender(Some(INTERVALO)) {
                    return fim;
                }
            }
            match sintetizar_trecho(self.motor, &trecho, &self.cancelar) {
                Ok(amostras) => {
                    if !amostras.is_empty() && !self.cancelar.load(Ordering::SeqCst) {
                        self.enfileirar(&amostras);
                    }
                }
                Err(erro) => eprintln!("narração: trecho ignorado: {erro}"),
            }
        }
        // Espera a fila tocar até o fim (pausa também segura aqui).
        while !self.player.empty() {
            if let Err(fim) = self.atender(Some(INTERVALO)) {
                return fim;
            }
        }
        None
    }

    fn enfileirar(&self, amostras: &[i16]) {
        let pontos: Vec<f32> = amostras
            .iter()
            .map(|&a| f32::from(a) / 32768.0)
            .collect();
        let fonte = SamplesBuffer::new(NonZero::<u16>::MIN, self.motor.taxa, pontos);
        self.player.append(fonte);
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::time::Instant;

    #[test]
    fn plain_text_remove_cabecalhos_negrito_codigo_links_e_listas() {
        let md = "# Título\n\nTexto **negrito**, `código` e [um link](https://exemplo.com).\n\n- item um\n* item dois\n2. item três\n";
        assert_eq!(
            plain_text(md),
            "Título\nTexto negrito, código e um link.\nitem um\nitem dois\nitem três"
        );
    }

    #[test]
    fn plain_text_pula_blocos_de_codigo() {
        let md = "Antes\n```rust\nlet x = 1;\n```\nDepois";
        assert_eq!(plain_text(md), "Antes\nDepois");
    }

    #[test]
    fn plain_text_trata_imagens_tabelas_regras_e_snake_case() {
        let md = "![Logo da UVA](img.png)\n\n---\n\n| a | b |\n|---|---|\n\nuse snake_case aqui";
        assert_eq!(plain_text(md), "Logo da UVA\na b\nuse snake case aqui");
    }

    #[test]
    fn dividir_em_trechos_respeita_limite_e_fronteiras_de_frase() {
        let longo = "Esta frase tem umas palavras para ocupar espaço. ".repeat(30);
        let trechos = dividir_em_trechos(&longo);
        assert!(trechos.len() > 1, "texto longo deveria gerar vários trechos");
        assert!(trechos.iter().all(|t| t.len() <= TAMANHO_TRECHO));
        assert!(trechos.iter().all(|t| t.ends_with('.')));
        assert_eq!(
            trechos.join(" "),
            longo.split_whitespace().collect::<Vec<_>>().join(" ")
        );
        assert!(dividir_em_trechos("   \n  ").is_empty());
    }

    #[test]
    fn dados_do_espeak_sao_encontrados_a_partir_do_target() {
        assert!(
            localizar_dados().is_some(),
            "espeak-ng-data não encontrado: o espeak-rs-sys precisa da feature `compile-espeak-intonations` (gera fonemas e dicionários); alternativamente defina ESTUDA_ESPEAK_DATA"
        );
    }

    #[test]
    fn sintetiza_ola_em_portugues_do_brasil() {
        let motor = motor().unwrap_or_else(|erro| panic!("eSpeak-ng não iniciou: {erro}"));
        assert!(
            motor.voz_pt_br,
            "voz pt-br ausente no build do eSpeak-ng: {}",
            motor.aviso.as_deref().unwrap_or("sem detalhe")
        );
        let fala = sintetizar("Olá, teste.")
            .unwrap_or_else(|erro| panic!("síntese falhou: {erro}"));
        assert!(fala.taxa > 0, "taxa de amostragem zerada");
        assert!(!fala.amostras.is_empty(), "síntese não produziu amostras");
    }

    #[test]
    fn sintese_cancelada_aborta_e_motor_continua_usavel() {
        let motor = motor().unwrap_or_else(|erro| panic!("eSpeak-ng não iniciou: {erro}"));
        let longo = "Uma frase qualquer para encher o trecho de texto. ".repeat(40);

        let completa = sintetizar_trecho(motor, &longo, &Arc::new(AtomicBool::new(false)))
            .unwrap_or_else(|erro| panic!("síntese completa falhou: {erro}"));

        let cancelar = Arc::new(AtomicBool::new(false));
        let disparo = {
            let cancelar = Arc::clone(&cancelar);
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(5));
                cancelar.store(true, Ordering::SeqCst);
            })
        };
        let inicio = Instant::now();
        let parcial = sintetizar_trecho(motor, &longo, &cancelar)
            .unwrap_or_else(|erro| panic!("síntese cancelada falhou: {erro}"));
        disparo.join().expect("thread de disparo");
        assert!(inicio.elapsed() < Duration::from_secs(20), "aborto demorou demais");
        assert!(
            parcial.len() < completa.len(),
            "o cancelamento não interrompeu a síntese ({} >= {})",
            parcial.len(),
            completa.len()
        );

        let depois = sintetizar_trecho(motor, "Olá, teste.", &Arc::new(AtomicBool::new(false)))
            .unwrap_or_else(|erro| panic!("motor não voltou a sintetizar: {erro}"));
        assert!(!depois.is_empty(), "motor não voltou a sintetizar após o aborto");
    }
}

/// Reprodução real pelo dispositivo de áudio padrão. Não roda por padrão, porque toca som.
/// Rode com: cargo test -p estuda-gpui reproducao_real -- --ignored --nocapture
#[cfg(test)]
mod reproducao_real {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    #[ignore]
    fn toca_frase_inteira_no_dispositivo_padrao() {
        let narrator = Narrator::new().expect("iniciar o eSpeak NG e o narrador");
        let inicio = Instant::now();
        narrator.speak("Olá. Este é o teste de narração do Estuda UVA.");
        // A reprodução começa de forma assíncrona: espera o narrador começar a falar.
        let limite = Instant::now() + Duration::from_secs(10);
        while !narrator.is_speaking() && Instant::now() < limite {
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(narrator.is_speaking(), "o narrador não começou a falar em 10 s");
        // Espera a frase terminar (limite de 30 s).
        let fim = Instant::now() + Duration::from_secs(30);
        while narrator.is_speaking() && Instant::now() < fim {
            std::thread::sleep(Duration::from_millis(50));
        }
        assert!(!narrator.is_speaking(), "a fala não terminou em 30 s");
        let decorrido = inicio.elapsed();
        assert!(decorrido >= Duration::from_millis(800), "fala curta demais: {decorrido:?}");
        println!("fala reproduzida em {decorrido:?}");
    }
}
