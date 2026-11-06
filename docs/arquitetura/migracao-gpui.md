# Migração Tauri → GPUI

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada.

## Conclusão curta

A migração está **no começo**. O que foi feito é separar o núcleo Rust da interface (`estuda-core`). A interface GPUI é só um **esqueleto**: uma janela com o texto "Estuda UVA", sem telas, sem uso de `estuda-core` e sem dados. O arquivo estava sendo alterado durante esta revisão (ver abaixo); o estado descrito é o da última leitura, 2026-11-06 00:55.

## O que já foi convertido

| Item | Evidência | Situação |
|---|---|---|
| Núcleo extraído do shell Tauri para o crate `estuda-core` | `app/crates/estuda-core/` (≈ 16 000 linhas Rust, 152 testes passam); `app/src-tauri/src/lib.rs` importa `estuda_core::{anthropic, archive, browser, …}`; `app/src-tauri/Cargo.toml` tem `estuda-core = { path = "../crates/estuda-core" }` | Feito, **não commitado** (os arquivos antigos `app/src-tauri/src/{archive,browser,…}.rs` constam como removidos no `git status`; `estuda-core/` e `app/Cargo.toml` aparecem como não rastreados) |
| Workspace Cargo de `app/` | `app/Cargo.toml`: membros `estuda-core`, `estuda-gpui`, `estuda-sync`; `exclude = ["src-tauri"]`; `resolver = "3"`, edition 2024, `rust-version = "1.90"` | Feito, não commitado |
| Crate de tipos de domínio | `crates/estuda-domain/` com `Confidence`, identificadores tipados, classificação privado/público | Feito, mas **não usado pelo núcleo** |
| Motor de leitura somente-leitura | `app/crates/estuda-sync/` | Em criação (ver [crates](crates.md#estuda-sync)) |

## O que permanece legado

- **Shell Tauri** `app/src-tauri/` (1 888 linhas em `src/lib.rs`): os 52 comandos, o `AppState`, a captura em lote do Canvas (`run_canvas_capture`, `capture_pdf_item`, `capture_page_item`), a importação individual (`import_material_inner`), a sincronização de calendário (`sync_calendar`) e as detecções de Chromium. Esta lógica de orquestração **não** está no núcleo.
- **Interface React/TypeScript** `app/src/` e seus testes `app/scripts/`.
- **Build do app**: `npm run tauri …` (via `@tauri-apps/cli`) e `app/package.json`.

O comentário em `app/Cargo.toml` registra a intenção: manter `src-tauri` "preservado para comparação de paridade".

## O que é GPUI hoje

- `app/crates/estuda-gpui/src/main.rs` (27–31 linhas, ainda em edição): `Application::new().run(...)` abre uma janela de 1280×820 com um `div` e o texto "Estuda UVA", via `gpui_kit::gpui`.
- `app/crates/estuda-gpui/Cargo.toml`: dependências `estuda-core` (não usada no código) e `gpui-kit = "0.9.0"`; binário `estuda-uva`. `app/Cargo.lock` passou a listar 26 pacotes `gpui*` (inclui `gpui-kit`, `gpui-component`) durante a revisão.
- Cronologia observada: às primeiras leituras o `main.rs` era `fn main() {}` e `Cargo.toml` não tinha `gpui`; `cargo check -p estuda-gpui --locked` falhou naquele momento ("cannot update the lock file"). Depois, `gpui-kit` e a janela apareceram. Nenhum `cargo` foi rodado sobre a versão nova.
- Dependências nativas e requisitos de plataforma do GPUI (macOS, Metal, Xcode, Linux/Windows): **Não verificado**. Nada neste repositório os documenta nem os exercita.

## Lacunas entre o legado e o necessário para o GPUI

Todas **Planejadas** (nenhuma implementada):

1. Extrair do shell Tauri para o núcleo (ou para um crate de serviços) a orquestração hoje presa a `tauri::State` e `AppHandle`: captura em lote, importação, calendário.
2. Substituir o canal de eventos Tauri (`emit`/`listen`) por um mecanismo independente de UI; hoje `SyncWatcher::start` já recebe um callback genérico, o que ajuda.
3. Decidir o destino do estado compartilhado (`AppState`) fora do Tauri.
4. Reproduzir as telas e o comportamento de acessibilidade da UI React. Ver [telas](../aplicativo/telas.md).
5. Decidir a relação entre `estuda-core`, `estuda-domain` e `estuda-sync` (ver [decisões pendentes](decisoes-pendentes.md)).

## Critério de paridade

Não há documento nem teste que defina "paridade" entre a UI React e uma futura UI GPUI. Proposta (**Planejada**): usar a lista de comandos de [referência/comandos](../referencia/comandos.md) como checklist funcional.
