//! Janela principal: barra de título, sidebar, cabeçalho e as oito telas do layout.

pub mod academico;
pub mod auxilio;
pub mod biblioteca;
pub mod data;
pub mod estudo;
pub mod kit;
pub mod leitor;
pub mod painel;
pub mod sincronizacao;
pub mod usuario;

use std::time::Duration;

use estuda_core::catalog::scanner::{LocalDocument, LocalDocumentContent};

use std::sync::Arc;

use crate::portal::Portal;
use crate::speech::Narrator;
use crate::services::{Discipline, Enrollment, Services};
use estuda_core::lyceum::PortalState;

use gpui_kit::component::input::{Input, InputEvent, InputState, TextareaState};
use gpui_kit::gpui::{
    AnyElement, ClickEvent, Context, Entity, Subscription, FontWeight, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::theme::{AZUL_MEDIO, LARANJA, Palette, hex};
use kit::{FONT_NUMERIC, FONT_SANS, Size, Variant, button, icon, progress};

/// Telas do menu lateral, na ordem do layout.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Painel,
    Academico,
    Sync,
    Biblioteca,
    Leitor,
    Estudo,
    Auxilio,
    Usuario,
}

impl Screen {
    pub const ALL: [Screen; 8] = [
        Screen::Painel,
        Screen::Academico,
        Screen::Sync,
        Screen::Biblioteca,
        Screen::Leitor,
        Screen::Estudo,
        Screen::Auxilio,
        Screen::Usuario,
    ];

    fn key(self) -> &'static str {
        match self {
            Screen::Painel => "painel",
            Screen::Academico => "academico",
            Screen::Sync => "sync",
            Screen::Biblioteca => "biblioteca",
            Screen::Leitor => "leitor",
            Screen::Estudo => "estudo",
            Screen::Auxilio => "auxilio",
            Screen::Usuario => "usuario",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Screen::Painel => "Painel",
            Screen::Academico => "Acadêmico",
            Screen::Sync => "Sincronização",
            Screen::Biblioteca => "Biblioteca",
            Screen::Leitor => "Leitor PDF→MD",
            Screen::Estudo => "Estudo",
            Screen::Auxilio => "Auxílio",
            Screen::Usuario => "Usuário",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Screen::Painel => "home",
            Screen::Academico => "school",
            Screen::Sync => "globe",
            Screen::Biblioteca => "inventory",
            Screen::Leitor => "eye",
            Screen::Estudo => "leaf",
            Screen::Auxilio => "volunteer",
            Screen::Usuario => "person",
        }
    }
}

/// Fase do fluxo de leitura dos portais (simulação da tela Sincronização).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SyncPhase {
    Idle,
    Login,
    Done,
}

/// Estado de cada fonte na tela Sincronização.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SrcState {
    Idle,
    Waiting,
    Done,
}

/// Estado da aplicação. Os campos seguem o `useState` do layout original.
pub struct AppView {
    pub screen: Screen,
    pub dark: bool,
    /// `painelLayout`: lista (padrão) ou quadro.
    pub quadro: bool,
    pub sel: usize,
    pub filter: usize,
    pub query: String,
    pub doc_idx: usize,
    pub conv: u8,
    pub estudo_tab: usize,
    pub card_idx: usize,
    pub flipped: bool,
    pub aux_tab: usize,
    pub task_idx: usize,
    pub chat_extra: Vec<(bool, String)>,
    pub msg: String,
    pub checks: [bool; 5],
    pub draft: String,
    pub answers: [Option<usize>; 3],
    pub sync_phase: SyncPhase,
    pub src_state: [SrcState; 5],
    pub reqs: u32,
    pub avoided: u32,
    pub log: Vec<(String, String)>,
    pub ram_limit: u32,
    pub prefs: [bool; 4],
    /// Serviços do acervo; `None` quando a pasta não pôde ser aberta.
    pub services: Option<Services>,
    /// Documentos do índice em RAM (sem varrer o disco a cada tela).
    pub library: Vec<LocalDocument>,
    pub library_status: String,
    /// Documento aberto no leitor, lido sob demanda.
    pub doc: Option<LocalDocumentContent>,
    /// RSS do processo em MB, atualizado periodicamente.
    pub ram_used_mb: Option<u64>,
    /// Campo de busca do cabeçalho (criado no primeiro render, pois precisa da janela).
    pub search: Option<Entity<InputState>>,
    search_sub: Option<Subscription>,
    /// Campo de tentativa do tutor e rascunho de Organizar.
    pub msg_input: Option<Entity<InputState>>,
    pub draft_input: Option<Entity<TextareaState>>,
    msg_sub: Option<Subscription>,
    draft_sub: Option<Subscription>,
    /// Sessão do Portal no Chromium do app (`None` se o runtime não puder ser criado).
    pub portal: Option<Arc<Portal>>,
    /// Narrador eSpeak NG, criado só na primeira vez que o aluno pede para ouvir.
    pub narrator: Option<Narrator>,
    /// Aviso de narração (voz ausente, motor indisponível).
    pub narration_note: Option<String>,
    /// Disciplinas do catálogo acadêmico lidas do banco sincronizado.
    pub disciplines: Vec<Discipline>,
    pub disciplines_status: String,
    /// Matrículas (trajetórias) do catálogo, para a tela Usuário.
    pub enrollments: Vec<Enrollment>,
}

impl AppView {
    pub fn new(services: Option<Services>) -> Self {
        Self {
            screen: Screen::Painel,
            dark: false,
            quadro: false,
            sel: 0,
            filter: 0,
            query: String::new(),
            doc_idx: 0,
            conv: 100,
            estudo_tab: 0,
            card_idx: 0,
            flipped: false,
            aux_tab: 0,
            task_idx: 0,
            chat_extra: Vec::new(),
            msg: String::new(),
            checks: [false; 5],
            draft: String::new(),
            answers: [None; 3],
            sync_phase: SyncPhase::Idle,
            src_state: [SrcState::Idle; 5],
            reqs: 0,
            avoided: 0,
            log: vec![("—".into(), "Nenhuma leitura nesta sessão".into())],
            ram_limit: 256,
            prefs: [true, true, true, true],
            services,
            library: Vec::new(),
            library_status: String::from("Acervo não carregado"),
            doc: None,
            ram_used_mb: None,
            search: None,
            search_sub: None,
            msg_input: None,
            draft_input: None,
            msg_sub: None,
            draft_sub: None,
            portal: None,
            narrator: None,
            narration_note: None,
            disciplines: Vec::new(),
            disciplines_status: String::from("Catálogo acadêmico não carregado"),
            enrollments: Vec::new(),
        }
    }

    /// Tela inicial pelo nome (`ESTUDA_TELA=academico`), usada só para conferir o layout.
    pub fn screen_from_name(name: &str) -> Option<Screen> {
        Screen::ALL.into_iter().find(|screen| screen.key() == name)
    }

    pub fn go(&mut self, screen: Screen) {
        self.screen = screen;
    }

    /// Texto e rótulo do indicador de sincronização na sidebar.
    fn sync_status(&self, p: &Palette) -> (&'static str, gpui_kit::gpui::Hsla) {
        match self.sync_phase {
            SyncPhase::Idle => ("Sem leitura nesta sessão", p.muted),
            SyncPhase::Login => ("Aguardando login", hex(LARANJA)),
            SyncPhase::Done => ("Leitura concluída", hex(crate::theme::PETROLEO)),
        }
    }

    fn titlebar(&self, p: &Palette) -> impl IntoElement {
        let control = |glyph: &'static str| {
            div()
                .text_size(px(12.))
                .text_color(p.muted)
                .cursor_pointer()
                .child(glyph)
        };
        div()
            .flex_none()
            .h(px(36.))
            .flex()
            .items_center()
            .justify_between()
            .pl(px(16.))
            .pr(px(14.))
            .bg(p.nav)
            .border_b_1()
            .border_color(p.nav_border)
            .text_size(px(12.))
            .text_color(p.muted)
            .child(
                div()
                    .id("drag-region")
                    .flex()
                    .flex_1()
                    .items_center()
                    .gap(px(8.))
                    .h_full()
                    .on_mouse_down(MouseButton::Left, |_, window: &mut Window, _| {
                        window.start_window_move();
                    })
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(p.text)
                            .child("Estuda UVA"),
                    )
                    .child("·")
                    .child("~/Faculdade"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(18.))
                    .child(
                        control("—")
                            .id("win-min")
                            .on_click(|_: &ClickEvent, window: &mut Window, _| {
                                window.minimize_window();
                            }),
                    )
                    .child(
                        control("▢")
                            .id("win-zoom")
                            .on_click(|_: &ClickEvent, window: &mut Window, _| {
                                window.zoom_window();
                            }),
                    )
                    .child(
                        control("✕")
                            .id("win-close")
                            .on_click(|_: &ClickEvent, window: &mut Window, _| {
                                window.remove_window();
                            }),
                    ),
            )
    }

    fn sidebar(&self, p: &Palette, cx: &mut Context<Self>) -> impl IntoElement {
        let (sync_label, sync_color) = self.sync_status(p);
        let ram_used = self.ram_used_mb.unwrap_or(0);
        let ram_pct = ram_used as f32 / self.ram_limit as f32 * 100.;

        let mut nav = div().flex().flex_col().gap(px(2.));
        for screen in Screen::ALL {
            let active = self.screen == screen;
            nav = nav.child(
                div()
                    .id(("nav", screen as usize))
                    .relative()
                    .flex()
                    .items_center()
                    .gap(px(10.))
                    .px(px(10.))
                    .py(px(9.))
                    .rounded(px(10.))
                    .text_size(px(14.))
                    .font_weight(if active { FontWeight::BOLD } else { FontWeight::MEDIUM })
                    .bg(if active { p.bg_alt } else { gpui_kit::gpui::transparent_black() })
                    .text_color(if active { p.text } else { p.muted })
                    .hover(|s| s.bg(p.bg_alt))
                    .cursor_pointer()
                    .child(
                        div()
                            .absolute()
                            .left(px(-12.))
                            .top(px(8.))
                            .bottom(px(8.))
                            .w(px(3.))
                            .rounded(px(3.))
                            .bg(hex(LARANJA))
                            .opacity(if active { 1.0 } else { 0.0 }),
                    )
                    .child(icon(screen.icon(), 18., if active { p.text } else { p.muted }))
                    .child(div().flex_1().child(screen.label()))
                    .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                        this.go(screen);
                        cx.notify();
                    })),
            );
        }

        div()
            .flex_none()
            .w(px(228.))
            .h_full()
            .flex()
            .flex_col()
            .gap(px(2.))
            .px(px(12.))
            .py(px(16.))
            .bg(p.nav)
            .border_r_1()
            .border_color(p.nav_border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.))
                    .px(px(8.))
                    .pt(px(4.))
                    .pb(px(18.))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(px(34.))
                            .rounded(px(10.))
                            .bg(hex(crate::theme::PETROLEO))
                            .text_color(hex(0xffffff))
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(14.))
                            .child("EU"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .text_size(px(15.))
                                    .child("Estuda UVA"),
                            )
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .text_color(p.muted)
                                    .w(px(150.))
                                    .child("2026.2 · Ciência da Computação"),
                            ),
                    ),
            )
            .child(nav)
            .child(div().flex_1().min_h(px(16.)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .p(px(12.))
                    .rounded(px(12.))
                    .bg(p.bg_alt)
                    .text_size(px(12.))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.))
                            .child(
                                div()
                                    .size(px(8.))
                                    .rounded_full()
                                    .bg(sync_color),
                            )
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(sync_label),
                            ),
                    )
                    .child(div().text_color(p.muted).child("Portais em somente leitura"))
                    .child(progress(p, ram_pct, hex(AZUL_MEDIO)))
                    .child(
                        div()
                            .font_family(FONT_NUMERIC)
                            .text_color(p.muted)
                            .child(format!("RAM {} / {} MB", ram_used, self.ram_limit)),
                    ),
            )
    }

    fn header(&self, p: &Palette, cx: &mut Context<Self>) -> impl IntoElement {
        let (title, subtitle) = data::title_for(self.screen.key());
        let theme_icon = if self.dark { "sun" } else { "moon" };
        div()
            .flex_none()
            .flex()
            .items_center()
            .gap(px(14.))
            .px(px(28.))
            .py(px(16.))
            .border_b_1()
            .border_color(p.border)
            .bg(p.bg)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .child(
                        div()
                            .text_size(px(22.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(p.text)
                            .child(title),
                    )
                    .child(
                        div()
                            .min_w_0()
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .text_size(px(13.))
                            .text_color(p.muted)
                            .child(subtitle),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .w(px(220.))
                    .px(px(12.))
                    .py(px(8.))
                    .rounded(px(10.))
                    .border_1()
                    .border_color(p.border)
                    .bg(p.surface)
                    .text_color(p.muted)
                    .child(icon("search", 16., p.muted))
                    .child(match &self.search {
                        Some(state) => div()
                            .flex_1()
                            .min_w_0()
                            .text_size(px(13.))
                            .text_color(p.text)
                            .child(Input::new(state).appearance(false).cleanable(false))
                            .into_any_element(),
                        None => div().flex_1().into_any_element(),
                    }),
            )
            .child(
                div()
                    .id("theme-toggle")
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(40.))
                    .rounded(px(10.))
                    .border_1()
                    .border_color(p.border)
                    .bg(p.surface)
                    .cursor_pointer()
                    .child(icon(theme_icon, 18., p.text))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.dark = !this.dark;
                        cx.notify();
                    })),
            )
            .child(
                button(p, "sync-btn", "Sincronizar", Variant::Primary, Size::Md, Some("bolt"))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.start_sync(cx);
                    })),
            )
    }

    fn content(&self, p: &Palette, cx: &mut Context<Self>) -> AnyElement {
        let body = match self.screen {
            Screen::Painel => painel::render(self, p, cx).into_any_element(),
            Screen::Academico => academico::render(self, p, cx).into_any_element(),
            Screen::Sync => sincronizacao::render(self, p, cx).into_any_element(),
            Screen::Biblioteca => biblioteca::render(self, p, cx).into_any_element(),
            Screen::Leitor => leitor::render(self, p, cx).into_any_element(),
            Screen::Estudo => estudo::render(self, p, cx).into_any_element(),
            Screen::Auxilio => auxilio::render(self, p, cx).into_any_element(),
            Screen::Usuario => usuario::render(self, p, cx).into_any_element(),
        };
        div()
            .id("content")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .px(px(28.))
            .pt(px(24.))
            .pb(px(48.))
            .child(body)
            .into_any_element()
    }

    /// Ação do botão Sincronizar: entra na tela de Sincronização e abre o Portal no
    /// Chromium do app. O login e o MFA são feitos pelo aluno na janela aberta.
    pub fn start_sync(&mut self, cx: &mut Context<Self>) {
        if matches!(self.sync_phase, SyncPhase::Login | SyncPhase::Done) {
            return;
        }
        self.screen = Screen::Sync;
        self.sync_phase = SyncPhase::Login;
        self.src_state = [SrcState::Waiting, SrcState::Idle, SrcState::Idle, SrcState::Idle, SrcState::Idle];
        self.add_log("Abrindo o Portal no Chromium do app…");
        let Some(portal) = self.portal.clone() else {
            self.sync_phase = SyncPhase::Idle;
            self.src_state[0] = SrcState::Idle;
            self.add_log("Não foi possível iniciar a sessão do Portal.");
            cx.notify();
            return;
        };
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn(async move { portal.open_lyceum() })
                .await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(status) => this.add_log(&format!("Portal aberto · {}", status.message)),
                    Err(error) => {
                        this.sync_phase = SyncPhase::Idle;
                        this.src_state[0] = SrcState::Idle;
                        this.add_log(&error);
                    }
                }
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }

    /// Botão "Concluí o login": confere a sessão real no Portal. Não lê notas nem gera arquivos.
    pub fn login_done(&mut self, cx: &mut Context<Self>) {
        let Some(portal) = self.portal.clone() else {
            return;
        };
        self.add_log("Conferindo a sessão do Portal…");
        cx.notify();
        cx.spawn(async move |this, cx| {
            let status = cx
                .background_spawn(async move { portal.lyceum_status() })
                .await;
            let _ = this.update(cx, |this, cx| {
                match status {
                    Some(status)
                        if matches!(status.state, PortalState::Authenticated | PortalState::Partial) =>
                    {
                        this.src_state[0] = SrcState::Done;
                        this.sync_phase = SyncPhase::Done;
                        this.add_log(&format!("Sessão confirmada · {}", status.message));
                    }
                    Some(status) => {
                        this.src_state[0] = SrcState::Waiting;
                        this.add_log(&status.message);
                    }
                    None => this.add_log("Abra o Portal antes de concluir o login."),
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn add_log(&mut self, message: &str) {
        self.log.insert(0, ("agora".into(), message.to_string()));
        self.log.retain(|(t, _)| t != "—");
        self.log.truncate(8);
    }
}

impl AppView {
    /// Lê o Markdown do documento escolhido e abre o leitor com a conversão animada.
    pub fn open_doc(&mut self, idx: usize, cx: &mut Context<Self>) {
        let Some(document) = self.library.get(idx).cloned() else {
            return;
        };
        self.stop_narration();
        self.screen = Screen::Leitor;
        self.doc = None;
        self.doc_idx = idx;
        let Some(services) = self.services.clone() else {
            self.library_status = "Acervo indisponível".into();
            cx.notify();
            return;
        };
        cx.spawn(async move |this, cx| {
            let relative = document.relative_path.clone();
            let result = cx
                .background_spawn(async move { services.read_document(&relative) })
                .await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(content) => {
                        this.doc = Some(content);
                        this.convert(cx);
                    }
                    Err(error) => this.library_status = error,
                }
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }

    /// Atualiza a leitura de memória do processo a cada 3 s, sem bloquear a interface.
    pub fn start_memory_monitor(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            loop {
                let used = cx.background_spawn(async { crate::services::resident_mb() }).await;
                let alive = this
                    .update(cx, |this, cx| {
                        this.ram_used_mb = used;
                        cx.notify();
                    })
                    .is_ok();
                if !alive {
                    break;
                }
                cx.background_executor().timer(Duration::from_secs(3)).await;
            }
        })
        .detach();
    }

    /// Lê as disciplinas do banco sincronizado em segundo plano (só leitura).
    pub fn refresh_disciplines(&mut self, cx: &mut Context<Self>) {
        let Some(services) = self.services.clone() else {
            self.disciplines_status = "Banco acadêmico indisponível".into();
            cx.notify();
            return;
        };
        let services_for_enrollments = services.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn(async move { services.disciplines() })
                .await;
            let enrollments = cx
                .background_spawn({
                    let services = services_for_enrollments.clone();
                    async move { services.enrollments() }
                })
                .await;
            let _ = this.update(cx, |this, cx| {
                if let Ok(list) = enrollments {
                    this.enrollments = list;
                }
                match result {
                    Ok(list) if list.is_empty() => {
                        this.disciplines_status = "Nenhuma disciplina sincronizada ainda".into();
                        this.disciplines = list;
                    }
                    Ok(list) => {
                        this.disciplines_status = format!("{} disciplinas no catálogo", list.len());
                        this.disciplines = list;
                    }
                    Err(error) => this.disciplines_status = error,
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Relê a pasta do acervo em segundo plano e atualiza o índice em RAM.
    pub fn refresh_library(&mut self, cx: &mut Context<Self>) {
        let Some(services) = self.services.clone() else {
            self.library_status = "Acervo indisponível".into();
            cx.notify();
            return;
        };
        self.library_status = "Lendo a pasta do acervo…".into();
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn(async move { services.refresh() })
                .await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(snapshot) => {
                        this.library_status = format!("{} documentos no índice", snapshot.documents.len());
                        this.library = snapshot.documents;
                    }
                    Err(error) => this.library_status = error,
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Animação de conversão do leitor (10 passos de 220 ms, como no layout).
    pub fn convert(&mut self, cx: &mut Context<Self>) {
        self.conv = 0;
        cx.notify();
        cx.spawn(async move |this, cx| {
            for step in 1..=10u8 {
                cx.background_executor()
                    .timer(Duration::from_millis(220))
                    .await;
                let alive = this
                    .update(cx, |this, cx| {
                        this.conv = step * 10;
                        cx.notify();
                    })
                    .is_ok();
                if !alive {
                    break;
                }
            }
        })
        .detach();
    }

    /// Narra o Markdown do documento aberto com a voz pt-BR embutida (eSpeak NG).
    pub fn narrate_doc(&mut self) {
        let Some(doc) = self.doc.as_ref() else {
            self.narration_note = Some("Abra um documento para ouvir.".into());
            return;
        };
        if self.narrator.is_none() {
            match Narrator::new() {
                Ok(narrator) => self.narrator = Some(narrator),
                Err(error) => {
                    self.narration_note = Some(error.to_string());
                    return;
                }
            }
        }
        if let Some(narrator) = &self.narrator {
            self.narration_note = narrator.aviso_voz().map(str::to_string);
            narrator.speak(&doc.markdown);
        }
    }

    /// Pausa ou retoma a narração em andamento.
    pub fn toggle_narration_pause(&mut self) {
        if let Some(narrator) = &self.narrator {
            if narrator.is_paused() {
                narrator.resume();
            } else {
                narrator.pause();
            }
        }
    }

    /// Interrompe a narração (também ao trocar de documento ou sair do leitor).
    pub fn stop_narration(&mut self) {
        if let Some(narrator) = &self.narrator {
            narrator.stop();
        }
    }

    /// Envia a tentativa do aluno e agenda a devolutiva do tutor (700 ms).
    pub fn send_message(&mut self, cx: &mut Context<Self>) {
        let text = self.msg.trim().to_string();
        if text.is_empty() {
            return;
        }
        self.msg.clear();
        self.chat_extra.push((true, text));
        cx.notify();
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(700))
                .await;
            let _ = this.update(cx, |this, cx| {
                this.chat_extra.push((
                    false,
                    "Boa. Antes de seguir: qual definição do material sustenta esse passo? Escreva com suas palavras.".into(),
                ));
                cx.notify();
            });
        })
        .detach();
    }
}

impl AppView {
    /// Cria o campo de busca na primeira renderização. Cada mudança de texto atualiza
    /// a consulta e leva à Biblioteca, como o `onQuery` do layout original.
    fn ensure_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search.is_some() {
            return;
        }
        let state = cx.new(|cx| InputState::new(window, cx).placeholder("Buscar no acervo"));
        let subscription = cx.subscribe_in(&state, window, |this: &mut AppView, state, event: &InputEvent, _window, cx| {
            if let InputEvent::Change = event {
                this.query = state.read(cx).value().to_string();
                this.screen = Screen::Biblioteca;
                cx.notify();
            }
        });
        self.search = Some(state);
        self.search_sub = Some(subscription);
    }

    /// Cria os campos de texto do tutor e do rascunho na primeira renderização.
    fn ensure_composers(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.msg_input.is_none() {
            let state = cx.new(|cx| InputState::new(window, cx).placeholder("Escreva sua tentativa ou dúvida"));
            let subscription = cx.subscribe_in(&state, window, |this: &mut AppView, state, event: &InputEvent, window, cx| {
                match event {
                    InputEvent::Change => {
                        this.msg = state.read(cx).value().to_string();
                    }
                    InputEvent::PressEnter { .. } => {
                        this.send_composer(window, cx);
                    }
                    _ => {}
                }
            });
            self.msg_input = Some(state);
            self.msg_sub = Some(subscription);
        }
        if self.draft_input.is_none() {
            let state = cx.new(|cx| {
                TextareaState::new(window, cx).placeholder(
                    "Estruture sua resposta aqui. O texto fica no acervo local; a entrega no Canvas é sempre feita por você.",
                )
            });
            let subscription = cx.subscribe_in(&state, window, |this: &mut AppView, state, event: &InputEvent, _window, cx| {
                if let InputEvent::Change = event {
                    this.draft = state.read(cx).value().to_string();
                    cx.notify();
                }
            });
            self.draft_input = Some(state);
            self.draft_sub = Some(subscription);
        }
    }

    /// Envia a mensagem digitada e limpa o campo (botão Enviar ou Enter).
    pub fn send_composer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(state) = self.msg_input.clone() {
            state.update(cx, |state, cx| state.set_value("", window, cx));
        }
        self.msg = String::new();
        self.send_message(cx);
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ensure_search(window, cx);
        self.ensure_composers(window, cx);
        let p = Palette::new(self.dark);
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(p.bg_alt)
            .text_color(p.text)
            .font_family(FONT_SANS)
            .text_size(px(14.))
            .child(self.titlebar(&p))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .child(self.sidebar(&p, cx))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .overflow_hidden()
                            .child(self.header(&p, cx))
                            .child(self.content(&p, cx)),
                    ),
            )
    }
}
