//! Tela Auxílio: tutor com narração, organização de atividades e simulado de treino.

use gpui_kit::component::input::{Input, Textarea};
use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, InteractiveElement, IntoElement, ParentElement, Styled, div,
    prelude::*, px,
};

use super::AppView;
use super::data::{CHECKS, DOCS, QUIZ, STEPS, SUBJECTS};
use super::kit::{self, Size, Variant, card, icon, segment, segmented};
use crate::theme::{AZUL_MEDIO, AZUL_PALIDO, LARANJA, PETROLEO, PETROLEO_DEEP, Palette, hex};

const AUX_TABS: [&str; 3] = ["Tutor", "Organizar", "Simulado"];

/// Atividades do painel lateral: AVA 1 e AVA 2 de cada uma das cinco primeiras
/// disciplinas, com a avaliação 2 de Cálculo inserida na terceira posição.
pub fn tasks() -> Vec<(String, String)> {
    let mut list = Vec::new();
    for (_, short, _) in SUBJECTS.iter().take(5) {
        list.push(("AVA 1".to_string(), short.to_string()));
        list.push(("AVA 2".to_string(), short.to_string()));
    }
    list.insert(2, ("Avaliação 2 (prova)".to_string(), "Cálculo".to_string()));
    list
}

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let all = tasks();
    let (kind, subj) = all[view.task_idx.min(all.len() - 1)].clone();

    let mut list = div().flex().flex_col().gap(px(6.)).w(px(240.)).flex_none();
    list = list.child(
        div()
            .px(px(4.))
            .pb(px(4.))
            .text_size(px(12.))
            .font_weight(FontWeight::BOLD)
            .text_color(p.muted)
            .child("ATIVIDADES E PROVAS"),
    );
    for (i, (k, s)) in all.iter().enumerate() {
        let selected = view.task_idx == i;
        list = list.child(
            div()
                .id(("task", i))
                .flex()
                .flex_col()
                .gap(px(2.))
                .px(px(12.))
                .py(px(10.))
                .rounded(px(10.))
                .border_1()
                .border_color(if selected { hex(AZUL_MEDIO) } else { gpui_kit::gpui::transparent_black() })
                .bg(if selected { p.card } else { gpui_kit::gpui::transparent_black() })
                .cursor_pointer()
                .child(div().text_size(px(13.)).font_weight(FontWeight::SEMIBOLD).child(k.clone()))
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(p.muted)
                        .line_height(px(16.))
                        .child(s.clone()),
                )
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.task_idx = i;
                    this.chat_extra.clear();
                    cx.notify();
                })),
        );
    }

    let mut tabs = segmented(p);
    for (i, label) in AUX_TABS.iter().enumerate() {
        tabs = tabs.child(segment(p, ("aux-tab", i), label, view.aux_tab == i).on_click(
            cx.listener(move |this, _: &ClickEvent, _, cx| {
                this.aux_tab = i;
                cx.notify();
            }),
        ));
    }

    let content = match view.aux_tab {
        0 => tutor(view, p, cx, &kind, &subj),
        1 => organizar(view, p, cx),
        _ => simulado(view, p, cx),
    };

    let banner = div()
        .flex()
        .items_center()
        .gap(px(10.))
        .px(px(14.))
        .py(px(10.))
        .rounded(px(12.))
        .bg(hex(AZUL_PALIDO))
        .text_color(hex(PETROLEO_DEEP))
        .text_size(px(13.))
        .child(icon("shield", 18., hex(PETROLEO_DEEP)))
        .child("O Auxílio trabalha só aqui dentro. Nada é enviado, confirmado ou agendado nos portais.");

    let right = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(14.))
        .child(banner)
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .flex_wrap()
                .child(
                    div()
                        .text_size(px(17.))
                        .font_weight(FontWeight::BOLD)
                        .child(format!("{kind} · {subj}")),
                )
                .child(div().flex_1())
                .child(tabs),
        )
        .child(content);

    div()
        .flex()
        .items_start()
        .gap(px(20.))
        .child(list)
        .child(right)
}

fn tutor(view: &AppView, p: &Palette, cx: &mut Context<AppView>, kind: &str, subj: &str) -> Div {
    let opening = [
        (false, format!("Vamos fazer juntos. Cole o enunciado de {kind} de {subj} ou diga o tema. Eu explico o conceito e você resolve cada passo.")),
        (true, "Ainda não li o enunciado no Canvas.".to_string()),
        (false, "Sem problema. Rode a sincronização para trazer o enunciado; até lá, revisamos o material ligado na aba Organizar.".to_string()),
    ];
    let messages: Vec<(bool, String)> = opening
        .into_iter()
        .chain(view.chat_extra.iter().cloned())
        .collect();

    let step_n = (1 + view.chat_extra.len() / 2).min(4);
    let mut steps = div().flex().gap(px(6.));
    for (i, label) in STEPS.iter().enumerate() {
        let done = i < step_n;
        steps = steps.child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(6.))
                .child(
                    div()
                        .h(px(4.))
                        .rounded(px(4.))
                        .bg(if done { hex(LARANJA) } else { p.border }),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(if done { p.text } else { p.muted })
                        .child(*label),
                ),
        );
    }

    let mut chat = div().flex().flex_col().gap(px(10.)).max_h(px(340.)).overflow_hidden();
    for (is_user, text) in messages {
        let bubble = div()
            .max_w(gpui_kit::gpui::relative(0.78))
            .px(px(14.))
            .py(px(10.))
            .rounded(px(14.))
            .text_size(px(14.))
            .line_height(px(21.))
            .bg(if is_user { hex(PETROLEO) } else { p.bg_alt })
            .text_color(if is_user { hex(0xffffff) } else { p.text })
            .child(text);
        chat = chat.child(
            div()
                .flex()
                .w_full()
                .when(is_user, |d| d.justify_end())
                .child(bubble),
        );
    }

    let field = match &view.msg_input {
        Some(state) => Input::new(state).appearance(false).into_any_element(),
        None => div().child("Escreva sua tentativa ou dúvida").into_any_element(),
    };
    let composer = div()
        .flex()
        .items_center()
        .gap(px(10.))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .px(px(14.))
                .py(px(10.))
                .rounded(px(10.))
                .border_1()
                .border_color(p.border)
                .bg(p.surface)
                .text_size(px(14.))
                .child(field),
        )
        .child(
            kit::button(p, "aux-send", "Enviar", Variant::Primary, Size::Md, Some("send")).on_click(
                cx.listener(|this, _: &ClickEvent, window, cx| {
                    this.send_composer(window, cx);
                }),
            ),
        );

    card(p)
        .p(px(18.))
        .gap(px(12.))
        .rounded(px(16.))
        .child(steps)
        .child(chat)
        .child(composer)
}

fn organizar(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let mut checklist = div().flex().flex_col().gap(px(2.));
    for (i, text) in CHECKS.iter().enumerate() {
        let on = view.checks[i];
        checklist = checklist.child(
            div()
                .id(("check", i))
                .flex()
                .items_center()
                .gap(px(10.))
                .px(px(4.))
                .py(px(8.))
                .text_size(px(13.))
                .text_color(if on { p.muted } else { p.text })
                .when(on, |d| d.line_through())
                .cursor_pointer()
                .child(
                    div()
                        .flex_none()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(20.))
                        .rounded(px(6.))
                        .border_2()
                        .border_color(if on { hex(AZUL_MEDIO) } else { p.border })
                        .bg(if on { hex(AZUL_MEDIO) } else { gpui_kit::gpui::transparent_black() })
                        .text_color(hex(0xffffff))
                        .text_size(px(12.))
                        .child(if on { "✓" } else { "" }),
                )
                .child(*text)
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.checks[i] = !this.checks[i];
                    cx.notify();
                })),
        );
    }

    let mut linked = div().flex().flex_col().gap(px(6.));
    for (i, doc) in [0usize, 1].into_iter().enumerate() {
        linked = linked.child(
            div()
                .id(("linked", i))
                .flex()
                .items_center()
                .gap(px(8.))
                .px(px(10.))
                .py(px(8.))
                .rounded(px(8.))
                .border_1()
                .border_color(p.border)
                .bg(p.surface)
                .text_size(px(12.))
                .cursor_pointer()
                .child(icon("link", 14., p.text))
                .child(DOCS[doc].1)
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.open_doc(doc, cx);
                })),
        );
    }

    let left = card(p)
        .p(px(18.))
        .gap(px(10.))
        .rounded(px(16.))
        .flex_1()
        .min_w(px(260.))
        .child(
            div()
                .text_size(px(13.))
                .font_weight(FontWeight::BOLD)
                .child("Checklist"),
        )
        .child(checklist)
        .child(
            div()
                .mt(px(6.))
                .text_size(px(13.))
                .font_weight(FontWeight::BOLD)
                .child("Materiais ligados"),
        )
        .child(linked);

    let right = card(p)
        .p(px(18.))
        .gap(px(10.))
        .rounded(px(16.))
        .flex_1()
        .min_w(px(260.))
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .child(div().text_size(px(13.)).font_weight(FontWeight::BOLD).child("Rascunho"))
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(p.muted)
                        .child(format!("salvo localmente · {} caracteres", view.draft.chars().count())),
                ),
        )
        .child(
            div()
                .flex_1()
                .min_h(px(300.))
                .p(px(12.))
                .rounded(px(10.))
                .border_1()
                .border_color(p.border)
                .bg(p.surface)
                .text_size(px(14.))
                .line_height(px(22.))
                .child(match &view.draft_input {
                    Some(state) => Textarea::new(state).appearance(false).into_any_element(),
                    None => div().into_any_element(),
                }),
        );

    div().flex().flex_wrap().gap(px(14.)).child(left).child(right)
}

fn simulado(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let mut col = div().flex().flex_col().gap(px(14.));
    let mut score = 0;
    for (qi, (question, options, correct)) in QUIZ.iter().enumerate() {
        let answer = view.answers[qi];
        if answer == Some(*correct) {
            score += 1;
        }
        let feedback = match answer {
            None => None,
            Some(a) if a == *correct => Some(("Correto".to_string(), hex(AZUL_MEDIO))),
            Some(_) => Some((format!("Resposta certa: {}", options[*correct]), hex(LARANJA))),
        };
        let mut grid = div().flex().flex_wrap().gap(px(8.));
        for (j, text) in options.iter().enumerate() {
            let (border, bg) = match answer {
                None => (p.border, p.surface),
                Some(_) if j == *correct => (hex(AZUL_MEDIO), hex(AZUL_PALIDO)),
                Some(a) if j == a => (hex(LARANJA), p.surface),
                Some(_) => (p.border, p.surface),
            };
            grid = grid.child(
                div()
                    .id(("quiz", qi * 10 + j))
                    .flex_basis(gpui_kit::gpui::relative(0.5))
                    .min_w(px(200.))
                    .px(px(12.))
                    .py(px(10.))
                    .rounded(px(10.))
                    .border_2()
                    .border_color(border)
                    .bg(bg)
                    .text_color(p.text)
                    .text_size(px(13.))
                    .cursor_pointer()
                    .child(*text)
                    .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                        if this.answers[qi].is_none() {
                            this.answers[qi] = Some(j);
                        }
                        cx.notify();
                    })),
            );
        }
        let mut item = card(p)
            .p(px(18.))
            .gap(px(10.))
            .rounded(px(16.))
            .child(
                div()
                    .text_size(px(14.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .line_height(px(20.))
                    .child(format!("{}. {}", qi + 1, question)),
            )
            .child(grid);
        if let Some((text, color)) = feedback {
            item = item.child(
                div()
                    .text_size(px(12.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color)
                    .child(text),
            );
        }
        col = col.child(item);
    }
    col.child(
        div()
            .text_size(px(13.))
            .text_color(p.muted)
            .flex()
            .flex_wrap()
            .child("Acertos: ")
            .child(
                div()
                    .font_family(kit::FONT_NUMERIC)
                    .font_weight(FontWeight::BOLD)
                    .text_color(p.text)
                    .child(format!("{score} / {}", QUIZ.len())),
            )
            .child(" · exercícios de treino, sem relação com notas oficiais"),
    )
}
