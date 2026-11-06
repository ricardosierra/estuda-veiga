# Referência de comandos

**Revisado em:** 2026-11-06
**Base:** commit `3811300` mais a árvore de trabalho não commitada daquele dia.

Esta página lista só o que existe no repositório: os comandos Tauri registrados em `app/src-tauri/src/lib.rs` e os comandos de linha de comando (Node, npm, cargo). Nada aqui é uma proposta. Onde um comando não foi executado, isso está dito.

Legenda de efeitos: **SQLite** = escreve no banco do app (`.estuda-uva.sqlite3` na raiz do acervo), **Arquivos** = cria ou altera arquivos no acervo, **Rede** = faz requisição HTTP a partir do backend Rust, **Chromium** = usa a janela Chromium controlada pelo app (login manual). "Lê" significa sem escrita. Qualquer comando que escreve em banco só deve ser disparado pelo usuário; esta documentação não autoriza executar SQL (ver [banco privado](../dados/banco-privado.md)).

## 1. Comandos Tauri (backend ↔ interface)

> **Aviso (2026-11-06):** o frontend React que chamava estes comandos foi removido no commit `aeab5f8`. A coluna "Chamado por" e a frase "a interface chama 50 deles" descrevem a interface **anterior** e só podem ser conferidas no histórico (`git show aeab5f8^:app/src/ui`). O shell `app/src-tauri` ainda registra os 52 comandos, mas hoje nenhuma interface os invoca; a interface GPUI atual só usa `CatalogService` de `estuda-core`.

A interface (anterior) chamava cada comando com `invoke("nome", { argumentos })` (`@tauri-apps/api/core`). Os argumentos Rust em `snake_case` chegam do TypeScript em `camelCase` (por exemplo `course_id` → `courseId`). Os parâmetros `State` e `AppHandle` são injetados pelo Tauri e não aparecem na chamada. Todo comando que retorna `Result<_, String>` rejeita a promessa com uma mensagem em português em caso de erro.

Estão registrados **52** comandos em `tauri::generate_handler!`. A interface React chamava 50 deles; `disconnect_canvas` e `local_catalog_snapshot` já estavam registrados sem uso (busca por `invoke` em `app/src`, antes da remoção). Contagem de 52 conferida por revisão independente em `generate_handler!` e `#[tauri::command]`.

Todas as operações que usam o Chromium passam por um `Mutex` (`canvas_operation`), então rodam uma de cada vez. O estado da sessão (`session`), as importações ativas (`active_imports`) e as capturas ativas (`active_captures`) ficam em memória (`AppState`) e se perdem ao fechar o app.

### 1.1 Canvas e navegador

| Comando | Argumentos | Retorno | Módulo | Efeitos | Chamado por |
|---|---|---|---|---|---|
| `canvas_status` | — | `CanvasStatus` (`state`, `message`, `currentUrl`) | `browser::CanvasSession::status` | Lê o estado da página aberta; sem sessão devolve "Canvas ainda não conectado." | atualização periódica (3 s), `App.tsx` |
| `connect_canvas` | — | `CanvasStatus` | `browser::CanvasSession::launch` | **Chromium**: abre janela visível com perfil próprio em `<app_data_dir>/chromium-profile` e navega para `https://uva.instructure.com/courses`. Idempotente se já conectado. Usa o executável salvo em `settings.chromium_executable` ou um dos caminhos conhecidos | Matérias, Agenda, Dados e sync, Acadêmico |
| `disconnect_canvas` | — | `()` | `browser::CanvasSession::close` | **Chromium**: fecha a sessão | nenhuma tela |
| `list_courses` | — | `Course[]` | `browser::list_courses` | **Chromium** (lê `/courses`); **SQLite**: grava `courses`. Erro se a lista vier vazia | Canvas e importação, Matérias, Dados e sync |
| `load_course` | `courseId` | `ModuleGroup[]` | `browser::load_course` | **Chromium**: lê os módulos do curso. Sem escrita | Canvas e importação, Matérias |
| `open_module_item` | `courseId`, `href` | `MaterialChoice[]` | `browser::open_module_item` | **Chromium**: abre um item de módulo e lista os PDFs encontrados (`allowed` indica se a origem é permitida). Não abre tarefas, fóruns ou avaliações | Canvas e importação, Matérias |
| `import_material` | `jobId` (UUID), `course`, `moduleTitle`, `title`, `semester` (opcional), `materialId` | `ImportedBundle` (`material`, `lessonMarkdown`) | `archive::import_pdf`, `download::download_pdf` | **Rede**: baixa o PDF (somente https, porta 443, sem credenciais na URL; hosts `uva.instructure.com`, `uva.br` e subdomínios, ou URL assinada do S3 da Amazon; até 60 MiB e 4 redirecionamentos, `download::validate_material_url`); o cookie do navegador é lido da sessão e usado só em memória. **Arquivos**: grava Markdown por página e aula em `materias/<curso>/materiais/` e atualiza os `README.md` de índice sem sobrescrever texto manual. **SQLite**: `materials`, `material_associations`, `practice_questions`. Reutiliza material já importado (mesma URL sem query) sem novo download. O PDF bruto não é mantido | Canvas e importação, Matérias |
| `cancel_import` | `jobId` | `()` | `lib.rs` (`active_imports`) | Sinal em memória que interrompe a importação antes de gravar | Canvas e importação, Matérias |
| `list_calendar_events` | `semester` | `CalendarEvent[]` | `storage::list_calendar_events` | Lê `academic_events` | Agenda (periódico) |
| `sync_calendar` | `semester` | `CalendarSyncReport` (`coursesChecked`, `eventsSaved`, `errors`) | `browser::list_assignment_events`, `calendar` | **Chromium** (somente leitura de prazos); **SQLite**: grava `courses` e substitui `academic_events` por matéria. Exige sessão autenticada e matérias atuais | Agenda (botão visível só em 2026.2) |

### 1.2 Captura em lote do Canvas

| Comando | Argumentos | Retorno | Módulo | Efeitos | Chamado por |
|---|---|---|---|---|---|
| `start_canvas_capture` | — | `CaptureRun` | `canvas_capture`, `browser`, `archive` | Recusa se já há captura ativa ou se o Canvas não está autenticado. **SQLite**: cria `capture_runs`, depois `capture_items`, `materials` e associações. **Arquivos**: Markdown por material. **Chromium** + **Rede** em segundo plano. Emite o evento `canvas-capture-updated` | Matérias (painel de captura) |
| `resume_canvas_capture` | `runId` | `CaptureRun` | idem | Retoma uma execução parcial; itens concluídos são preservados | Matérias |
| `cancel_canvas_capture` | `runId` | `()` | `lib.rs` (`active_captures`) | Sinal em memória; o item atual termina com segurança | Matérias |
| `canvas_capture_status` | `runId` (opcional) | `CaptureRun \| null` | `storage` | Lê a execução (a mais recente se `runId` for nulo). **Pode escrever** em `capture_runs` ao marcar como interrompida uma execução que não está mais ativa | Matérias, `App.tsx` |
| `canvas_capture_items` | `runId`, `offset`, `limit` | `CaptureItem[]` | `storage::list_capture_items` | Lê; `limit` é limitado a 200 | Matérias |
| `retry_canvas_capture_item` | `runId`, `referenceKey` | `CaptureRun` | `canvas_capture`, `storage` | Só para item `failed` com `retryable`; **SQLite** e novo processamento (Chromium, Rede, Arquivos) | Matérias |
| `get_canvas_capture_limits` | — | `CanvasCaptureLimits` | `storage::get_capture_limits` | Lê `settings.canvas_capture_limits` (padrão se ausente ou inválido) | Matérias |
| `set_canvas_capture_limits` | `limits` | `CanvasCaptureLimits` | `storage::save_capture_limits` | Valida e **grava** `settings.canvas_capture_limits` | Matérias |

Nenhum comando desta seção foi **verificado em execução real**: o estado de implementação registra que ainda não houve varredura autenticada pelo app.

### 1.3 Portal/Lyceum e leitura acadêmica do Canvas

| Comando | Argumentos | Retorno | Módulo | Efeitos | Chamado por |
|---|---|---|---|---|---|
| `open_lyceum` | — | `PortalStatus` | `browser::open_lyceum` | **Chromium**: abre (ou reaproveita) a sessão e uma segunda página em `https://portalaluno.uva.br/Aluno/PortalAluno`; se o portal redirecionar para fora dos domínios UVA permitidos, fecha a página e devolve erro. O login e o MFA são manuais; o app não lê credenciais | Acadêmico |
| `lyceum_status` | — | `PortalStatus` | `browser::lyceum_status` | Lê o estado; a URL devolvida passa por `lyceum::sanitize_source_url` (sem query e fragmento; fora dos domínios UVA vira erro) | Acadêmico, atualização periódica |
| `refresh_lyceum_catalog` | — | `PortalCapture` | `browser::refresh_lyceum_catalog`, `lyceum::persist_capture` | **Chromium**: navega por links acadêmicos oficiais, até 50 páginas, em modo de leitura. **Arquivos**: cria `materias/catalogo/capturas/lyceum-<id>.json` e `.md`, sem substituir arquivos existentes. **SQLite**: sincroniza os arquivos novos. Recusa gravar se nenhuma página legível foi lida | Acadêmico |
| `refresh_canvas_academic` | `options` (`grades`, `forums`, `announcements`, `inbox`) | `CanvasAcademicCapture` | `browser::capture_canvas_academic`, `canvas_api` | Exige ao menos uma opção e sessão autenticada. **Chromium**: lê pela API do Canvas com `fetch` (método padrão GET) executado na própria página, só em `/api/v1/` da mesma origem e com limites (40 cursos, 10 páginas por lista, 60 tópicos por curso, 2.000 mensagens por tópico, 100 conversas). Conversas são lidas com `auto_mark_as_read=false`; nada é postado nem marcado como lido. **Arquivos**: `materias/catalogo/capturas/canvas-<uuid>.json` e `.md`. **SQLite**: sincroniza | Acadêmico |

Nenhum comando desta seção foi **verificado em execução real**; a captura do Lyceum e a leitura da API dependem de sessão autenticada. O app não envia atividades, não agenda provas e não altera registros institucionais: não há comando que faça isso.

### 1.4 Acervo, sincronização e Git

| Comando | Argumentos | Retorno | Módulo | Efeitos | Chamado por |
|---|---|---|---|---|---|
| `sync_local_catalog` | — | `SyncReport` | `sync::SyncEngine::reconcile_all` | **SQLite** e **Arquivos**: reconcilia Markdown/JSON/SQLite, escreve espelhos em `.estuda-uva/mirrors/` e propaga edições. Chamado na abertura do app | Dados e sync; abertura |
| `local_sync_conflicts` | — | `SyncConflict[]` | `sync::open_conflicts` | Lê `sync_conflicts` | Dados e sync |
| `edit_synced_document` | `relativePath`, `content` | `SyncReport` | `sync::set_content` + `reconcile_all` | **SQLite** e **Arquivos**: grava a edição nas três representações; divergência vira conflito, não sobrescrita | Dados e sync |
| `resolve_local_sync_conflict` | `relativePath`, `content` | `SyncReport` | `sync::resolve_conflict` + `reconcile_all` | **SQLite** e **Arquivos**: aplica a versão escolhida e limpa o conflito | Dados e sync |
| `local_catalog_snapshot` | — | `LocalCatalogSnapshot` | `catalog::scanner::snapshot` | Lê o índice em memória. **Não é chamado pela interface** | nenhuma tela |
| `refresh_local_catalog` | — | `LocalCatalogSnapshot` | `catalog::scanner::refresh` | Varre a pasta do acervo (até 10.000 arquivos, profundidade 16); sem escrita | Dados e sync; estudo |
| `read_local_document` | `relativePath` | `LocalDocumentContent` (`document`, `markdown`, `warnings`) | `catalog::scanner::read_document` | Lê e converte (Markdown, texto, JSON, PDF, DOCX, Pages) até 64 MiB; sem escrita | Dados e sync; aulas |
| `read_local_asset` | `relativePath` | `number[]` (bytes) | `catalog::scanner::read_asset` | Lê imagem local até 16 MiB | Aula guiada, Dados e sync |
| `search_local_library` | `query` | `LocalSearchResponse` | `catalog::scanner::search` | Lê; até 100 resultados | Dados e sync |
| `academic_catalog` | — | `AcademicCatalog` | `sync::reconcile_all`, `catalog::academic::load` | Reconcilia antes de ler, então **também escreve** (SQLite e espelhos). Falha se houver erro em `materias/catalogo/` | Estudar, Matérias, Acadêmico |
| `repository_status` | — | `RepositoryStatus` | `repository::repository_status` | Lê o estado do Git do acervo; não faz stage, commit nem push | Dados e sync |
| `set_library_path` | `path` | `()` | `storage::set_library_path`, `sync`, `catalog`, `sync_watcher` | Pasta precisa existir. **SQLite**: passa a usar `<pasta>/.estuda-uva.sqlite3`; se ele não existir, **copia** o banco atual para lá (API de backup do SQLite). Reaponta sincronizador, catálogo e monitor; reverte tudo se algum falhar | Dados e sync |
| `set_chromium_path` | `path` | `()` | `lib.rs`, `storage::set_setting` | **SQLite**: grava `settings.chromium_executable`. Aceita um `.app` e procura `Contents/MacOS/Chromium` ou `Google Chrome` | Dados e sync |

O monitor de arquivos (`sync_watcher`) não é um comando: é iniciado em `run()` e emite `local-sync-updated`.

### 1.5 Estudo local (aula, tutor, prática, histórico)

Nenhum comando desta seção usa rede nem IA.

| Comando | Argumentos | Retorno | Módulo | Efeitos | Chamado por |
|---|---|---|---|---|---|
| `overview` | — | `Overview` (contagens, `accuracyPercent`, `libraryPath`) | `storage::overview` | Lê `courses`, `materials`, `study_sessions` | atualização periódica |
| `list_materials` | — | `ImportedMaterial[]` | `storage::list_materials` | Lê `materials` | atualização periódica, captura |
| `get_lesson` | `materialId` | `String` (Markdown) | `storage`, arquivo da aula | Lê o arquivo da aula salva | Aula guiada |
| `start_practice` | `materialId` | `PracticePrompt[]` | `storage::load_questions` | Lê `practice_questions` | Aula guiada (materiais importados) |
| `submit_practice` | `materialId`, `answers` | `PracticeResult` | `lesson::grade_answers`, `storage::record_session` | **SQLite**: grava `study_sessions` (resultado da prática) | Aula guiada |
| `begin_lesson_session` | `resourcePath`, `courseId` (opcional), `courseTitle`, `materialTitle`, `sectionCount` | `LessonSession` | `storage::begin_lesson_session` | **SQLite**: reaproveita a sessão não concluída mais recente do mesmo `resourcePath` (atualiza `section_count`) ou cria uma nova em `lesson_sessions` | Matérias, Aula guiada |
| `save_lesson_progress` | `sessionId`, `sectionIndex`, `sectionCount`, `completed` | `LessonSession` | `storage::save_lesson_progress` | **SQLite**: atualiza `lesson_sessions` | Aula guiada |
| `get_lesson_session` | `sessionId` | `LessonSession \| null` | `storage` | Lê | Matérias, histórico |
| `build_lesson_activity` | `section` (`TutorSectionInput`), `sessionId` | `TutorActivity` | `tutor::build_tutor_activity` | Cálculo determinístico, sem I/O; rejeita seção acima de 1 MiB, título acima de 512 bytes ou página 0 | Aula guiada |
| `assess_lesson_answer` | `activity`, `response` | `TutorAssessment` | `tutor::assess_tutor_answer` | Cálculo determinístico, sem I/O | Aula guiada |
| `save_lesson_interaction` | `sessionId`, `activity`, `response`, `assessment` | `LessonInteraction` | `storage::save_lesson_interaction` | **SQLite**: grava `lesson_interactions` (resposta digitada e trecho da fonte) | Aula guiada |
| `list_lesson_interactions` | `sessionId` | `LessonInteraction[]` | `storage` | Lê | Aula guiada |
| `get_lesson_interaction` | `sessionId`, `activityId`, `currentSourceSha256` | `LessonInteraction \| null` | `storage` | Lê; sinaliza se o Markdown mudou desde a tentativa | Aula guiada |
| `list_study_history` | `courseTitle` (opcional), `limit` (opcional, padrão 200) | `StudyHistoryEntry[]` | `storage::list_study_history` | Lê `lesson_sessions` e `study_sessions` | Histórico, Matérias |

### 1.6 Resumos com IA (envia texto a terceiro)

| Comando | Argumentos | Retorno | Módulo | Efeitos | Chamado por |
|---|---|---|---|---|---|
| `get_subject_summary_snapshot` | `courseId`, `courseTitle`, `semester` | `SubjectSummarySnapshot` (`sources`, `currentSourceHash`, `latestSummary`) | `study_summaries` | Atualiza o catálogo em memória e lista as fontes que **seriam** enviadas; lê o último resumo salvo. Sem rede, sem escrita de resumo | Matérias → aba Resumos |
| `generate_subject_summaries` | `courseId`, `courseTitle`, `semester`, `confirmedSourceHash` | `StudySummary` | `study_summaries`, `anthropic` | Recusa se o hash das fontes mudou desde a confirmação. **Rede**: envia o texto dos materiais à API da Anthropic (`https://api.anthropic.com/v1/messages`). A chave, o modelo e o workspace vêm de um `.env` fora do repositório, lido pelo backend (`AnthropicConfig::read_default`); o valor da chave não é documentado. **SQLite**: grava `study_summaries` (textos, modelo, tokens, manifesto) | Matérias → aba Resumos |

**Não verificado em execução real.** Na checagem de 2026-11-06 a chave configurada recebeu HTTP 400 (sem `anthropic-workspace-id`); os 8 testes do cliente usam servidor simulado. Detalhes em [features/README.md](../features/README.md).

### 1.7 Cálculo local

| Comando | Argumentos | Retorno | Módulo | Efeitos | Chamado por |
|---|---|---|---|---|---|
| `calculate_final_grade` | `input` (`kind`, `a1`, `a2`, `a3`, `a4`) | `FinalGradeResult` | `grading::final_grade` | Cálculo puro; sem I/O, sem rede. É estimativa; vale o boletim do Lyceum | Acadêmico |

### 1.8 Eventos emitidos pelo backend (não são comandos)

| Evento | Quando | Quem escuta |
|---|---|---|
| `local-sync-updated` (`SyncReport`) | o monitor reconcilia depois de alterações em `materias` ou `docs` | (React removido) |
| `canvas-capture-updated` (`CaptureRun`) | mudança de estado ou progresso da captura em lote | (React removido) |

## 2. Comandos de linha de comando

Execute a partir da **raiz do repositório** (`Faculdade/`), salvo indicação. Não existe `Cargo.toml` na raiz: todo comando `cargo` precisa de `--manifest-path`.

### 2.1 Gerador e validador do acervo (`tools/acervo`, Node 24, sem dependências externas)

| Comando | O que faz | Escreve? | Verificado em 2026-11-06 |
|---|---|---|---|
| `node tools/acervo/build.mjs --validar` | Valida os registros de dados (campos, `null` com pendência, referências, fontes existentes, dados sensíveis) | Não | Sim: "OK: 234 registros válidos em dados/." |
| `node tools/acervo/build.mjs --verificar-texto` | Lista binários rastreados (erro) e locais (aviso) | Não | Sim: "0 erro(s), 0 aviso(s)", código de saída 0 |
| `node tools/acervo/build.mjs` | Valida e **gera** `materias/catalogo/*.json` e `materias/catalogo/curriculos/`, `build/acervo.sqlite3` e `build/visualizacoes/*.md` | **Sim** (arquivos gerados, ignorados pelo Git; recria o SQLite de `build/`) | Não executado: sobrescreve saídas geradas |
| `node tools/acervo/src/pdf-para-md.mjs <arquivo.pdf> <saida.md> [fonte]` | Converte um PDF em Markdown por página com `pdftotext -layout` (precisa de `pdftotext` e `pdfinfo`, do pacote poppler). Não remove o PDF. Páginas sem texto são sinalizadas | **Sim** (cria `<saida.md>`) | Não executado |
| `node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs` | Testes do gerador e da política de texto. Constroem acervos mínimos em pastas temporárias; um teste valida os dados reais, só leitura | Só em diretórios temporários | Sim: 13 passaram, 0 falharam |
| `node tools/acervo/src/separar-privado.mjs` | Simula (padrão) o plano de separar `privado/` do restante e imprime o plano | Não (sem `--aplicar`) | Não executado |
| `node tools/acervo/src/separar-privado.mjs --aplicar` | **Move** arquivos para `privado/` e reescreve links e caminhos de fonte | **Sim, move arquivos** | Não executado. Parece já ter sido aplicado na árvore de trabalho atual (a pasta `privado/` existe); não rode de novo sem revisar o plano |
| `node tools/acervo/migrar-legado.mjs --origem <pasta>` | Migração única do formato antigo para `dados/` | **Sim** (cria registros em `dados/`) | Não executado. O README do gerador diz que já foi executado e é mantido como registro |

Referência: [tools/acervo/README.md](../../tools/acervo/README.md) e [dados/LEIA-ME.md](../../dados/LEIA-ME.md).

### 2.2 Frontend e shell Tauri (removido)

`app/package.json`, `app/package-lock.json`, `app/scripts/`, `app/src/`, `app/index.html` e `app/vite.config.ts` foram apagados no commit `aeab5f8`. Portanto **não existem mais** `npm install`, `npm test`, `npm run build`, `npm run dev` e `npm run tauri dev/build` a partir de `app/`. Em 2026-11-06, antes da remoção, `npm test` passou (40 testes) e `npm run build` não foi executado. `app/src-tauri/tauri.conf.json` ainda define `frontendDist: "../dist"`, e `app/dist` ainda existe em disco sem ser gerado por nenhum comando do repositório. Para recuperar os testes antigos: `git show aeab5f8^:app/scripts`.

### 2.3 Rust

Três manifestos independentes (não há workspace na raiz):

| Manifesto | Pacotes | Observação |
|---|---|---|
| `app/Cargo.toml` | workspace com `estuda-core`, `estuda-gpui` e `estuda-sync` (`exclude = ["src-tauri"]`) | `app/Cargo.lock` ainda **não está versionado** |
| `crates/estuda-domain/Cargo.toml` | `estuda-domain`, com `[workspace]` próprio e sem dependências | |
| `app/src-tauri/Cargo.toml` | `estuda-uva` (shell Tauri) | Fora do workspace; depende de `estuda-core` por caminho |

| Comando | O que faz | Verificado em 2026-11-06 |
|---|---|---|
| `cargo test --locked --offline --manifest-path app/Cargo.toml -p estuda-core --lib` | Testes do núcleo | Sim: **152 passaram**, 0 falharam (antes do esquema v8 e de testes novos; um revisor contou 154 no código posterior). Usa SQLite e pastas temporárias. O teste `launches_real_isolated_chromium_and_reads_fixture_dom` abre um Chromium real (headless, perfil temporário) em `/opt/homebrew/bin/chromium` e **falha por pânico** se esse arquivo não existir |
| `cargo test --locked --offline --manifest-path crates/estuda-domain/Cargo.toml` | Testes do crate de domínio | Sim: **14 passaram** na execução; o crate foi alterado depois e tem 17 `#[test]` (4 + 4 + 9) |
| `cargo check --locked --offline --manifest-path app/Cargo.toml -p estuda-gpui` | Compila a interface GPUI (binário `estuda-uva`, `gpui-kit = "0.9.0"`) | **Falhou** duas vezes: primeiro com "cannot update the lock file" (lock desatualizado, numa versão anterior do crate) e depois com `E0583` (módulo `speech` ausente, edição não commitada de outra sessão). Sem `--locked` não foi tentado |
| `cargo fmt --manifest-path app/Cargo.toml --all -- --check` | Verifica a formatação | Executado: **terminou com código 1** (há diferenças de formatação; a saída lista arquivos de `estuda-core` como `study_summaries.rs` e também de `crates/estuda-domain/src`, sem investigação da causa) |
| `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib` | Documentado em `app/README.md` | Não executado. `app/src-tauri/src/lib.rs` não contém nenhum `#[test]`, então esse comando não exerce nada do núcleo, que foi extraído |
| `cargo check --offline --manifest-path app/src-tauri/Cargo.toml` | Compila o shell Tauri | Não executado |

Dependências nativas do GPUI em cada sistema: **não verificadas**. `app/crates/estuda-gpui/Cargo.toml` declara `gpui-kit`.

## 3. Resumo do que foi e do que não foi validado

- **Validado nesta revisão:** validação de dados (234 registros), política de texto, 13 testes Node do gerador, 40 testes npm (antes da remoção do frontend), 152 testes do `estuda-core` e 14 do `estuda-domain` (contagens anteriores a alterações posteriores).
- **Falhou ou apontou problema:** `cargo check` do `estuda-gpui` (lockfile desatualizado e, depois, módulo ausente) e `cargo fmt --check`.
- **Não executado:** `npm run build`, `npm run tauri dev/build`, qualquer comando que escreve (geração de `build/`, conversão de PDF, `separar-privado.mjs --aplicar`), `cargo` do `src-tauri`.
- **Sem verificação em execução real:** tudo que depende de sessão autenticada (Canvas, Portal/Lyceum) e da API da Anthropic. Os testes automatizados usam fixtures e servidores simulados e não comprovam capturas reais.

## Referências de código

- `app/src-tauri/src/lib.rs` — comandos, `AppState`, `run()` e `generate_handler!`.
- `app/crates/estuda-core/src/` — módulos que executam cada comando.
- `git show aeab5f8^:app/src/ui/App.tsx` e demais `.tsx` — chamadas `invoke` da interface removida.
- `app/src-tauri/tauri.conf.json` — configuração do shell; `app/crates/estuda-gpui/` — interface atual.
- `tools/acervo/` — gerador, validador e seus testes.
