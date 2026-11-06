# Crates e módulos

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada. Contagens de testes obtidas por `grep -c '#\[test\]\|#\[tokio::test\]'` e confirmadas pelas execuções de 2026-11-06 (ver [testes](../desenvolvimento/testes.md)).

Convenção de caminhos: `app/crates/estuda-core/src/` é abreviado como `core/src/`.

## Mapa

| Crate / pacote | Manifesto | Estado |
|---|---|---|
| `estuda-domain` | `crates/estuda-domain/Cargo.toml` (workspace próprio, sem dependências) | Implementado; 14 testes |
| `estuda-core` | `app/crates/estuda-core/Cargo.toml` | Implementado; 156 testes (6/11/2026, sem o teste do Chromium); não commitado |
| `estuda-sync` | `app/crates/estuda-sync/Cargo.toml` | Código presente (15 módulos, 112 funções `#[test]`); **não compilado nem testado por esta revisão; não integrado**; ainda em alteração |
| `estuda-gpui` | `app/crates/estuda-gpui/Cargo.toml` | Esqueleto (janela vazia); em alteração durante a revisão |
| `estuda-uva` (shell Tauri) | `app/src-tauri/Cargo.toml` | Implementado; 0 testes Rust |
| Frontend React | `app/package.json` | Implementado; 40 testes Node |
| Gerador Node | `tools/acervo/` | Implementado; 13 testes Node |

## `estuda-domain`

- **Responsabilidade:** vocabulário puro do domínio, sem I/O (`crates/estuda-domain/src/lib.rs`).
- **API pública:**
  - `confidence`: `enum Confidence { Confirmed, Estimated, Unknown, Conflicting }`, `is_official()`, `reconcile(other, values_equal)`.
  - `ids`: `enum IdError`, tipos de identificador por macro `kinded_id!` (com `parse` e `as_str`) e `LmsCourseId::canvas`, `provider`, `id`. Formato `<namespace>:<tipo>:<slug>`, até 200 caracteres, só `[a-z0-9:_.-]`.
  - `publication`: `enum Layer { LocalOnly, Personal, Generated, Shareable, NeedsReview }`, `struct PublicationPolicy { allow_personal }` (`Default` nega pessoal; `PublicationPolicy::OWNER` permite, política do dono de 6/11/2026), `validate_relative_path`, `classify`, `check_publishable(paths, reviewed, policy) -> Vec<Violation>`.
- **Erros:** `IdError`, `PathError` (`Empty`, `Absolute`, `Traversal`, `Backslash`, `Nul`).
- **Persistência / concorrência / estado:** nenhuma.
- **Testes:** `confidence.rs` 4, `ids.rs` 4, `publication.rs` 9 (total 17).
- **Limitações:** a classificação é só por caminho; não inspeciona conteúdo. As listas de prefixos (`PERSONAL_PREFIXES`/`LOCAL_ONLY_PREFIXES`, `SHAREABLE_PREFIXES`…) são fixas no código e incluem `biblioteca/` e `schemas/`, que não existem no repositório hoje. **Nenhum outro crate de `app/` a usa além de `estuda-sync` (declarada, ver abaixo); `estuda-core` não depende dela.**

## `estuda-core`

`core/src/lib.rs` declara 19 módulos públicos e um módulo de testes de integração (11 testes). Dependências principais (`Cargo.toml`): `rusqlite` 0.37 (bundled, backup), `tokio`, `reqwest` (rustls), `chromiumoxide` 0.9.1, `notify` 8.2, `pdf-extract`, `docx-lite`, `iwork`, `scraper`, `serde`, `sha2`, `thiserror`, `uuid`, `zip`. Dev: `lopdf`, `tempfile`.

### Persistência e sincronização

| Módulo | Responsabilidade e API | Estado / concorrência | Erros | Testes |
|---|---|---|---|---|
| `storage.rs` | `AcademicStore`: `open`, `open_workspace`, cursos, materiais, associações, execuções e itens de captura, sessões de aula, interações, resumos, eventos de calendário, configurações. `CURRENT_SCHEMA_VERSION = 7`. | `Mutex<Connection>`, `Mutex<PathBuf>` (acervo e banco). SQLite em WAL. | `StorageError { Io, Sql }`; recusa banco com `user_version` maior que 7. | 19 |
| `schema.rs` | Esquema SQLite v8 numa `BASELINE` só, sem `ALTER TABLE`; `CURRENT_VERSION`, `BASELINE_TABLES`. Banco de versão menor é movido para `*.bak.sqlite3` e regenerado por `storage::open_connection`. |
| `sync.rs` | `SyncEngine`: `open`, `set_workspace`, `reconcile_all`, `read_content`, `create_content`, `set_content`, `open_conflicts`, `resolve_conflict`, `read_conflicts`, `read_academic_catalog`. Funções `reconcile` (três vias) e tipos `SurfaceValue`, `Reconciliation`, `SyncReport`, `SyncConflict`. | `Mutex<PathBuf>` + `Mutex<Connection>`. Arquivos até 64 MiB. Espelhos em `.estuda-uva/mirrors/`. | `Result<_, String>` | 11 |
| `sync_watcher.rs` | `SyncWatcher::start(root, engine, on_sync)`, `set_root`; `Drop` encerra a thread. | Thread própria, `mpsc`, `RwLock`, `Mutex`, `AtomicBool`; silêncio de 500 ms; observa a raiz (sem recursão), `materias/` e `docs/` (recursivo). | `Result<_, String>` | 8 |
| `repository.rs` | `repository_status(root) -> RepositoryStatus`: executa `git -C <raiz> status --short … --ignored=matching`. Somente leitura. | Processo externo `git`. | Sem Git ⇒ `RepositoryStatus.error` preenchido, não é falha. | 2 |

Esquemas das tabelas: [dados/banco-privado.md](../dados/banco-privado.md).

### Catálogo local (`core/src/catalog/`)

| Arquivo | Responsabilidade | Testes |
|---|---|---|
| `scanner.rs` | `CatalogService` (`new`, `set_root`, `snapshot`, `refresh`, `read_document`, `read_asset`, `search`), `scan_library`, `read_document`. `Mutex` na raiz e no índice. Limites: `MAX_FILES` 10 000, `MAX_DEPTH` 16, 64 MiB por documento, 16 MiB por asset, 128 MiB de texto indexado, 100 resultados. | 5 |
| `documents.rs` | Extração de texto por formato (PDF, DOCX, Pages, HTML, JSON, texto). Limite de 64 MiB. | 6 |
| `classification.rs` | `classify_path -> PathClassification { course, semester, category }`. Lista fixa de seis matérias conhecidas em `known_course`. | 5 |
| `academic.rs` | Esquema e projeção relacional `academic_*`: `initialize_schema`, `replace_projection`, `read_projection`, `load_documents`, `load`. Lê `materias/catalogo/{curriculos/*,disciplinas,relacoes,trajetorias}.json`. | 2 |
| `guidance.rs` | `create_missing_guidance(root)`: cria `AGENTS.md` e `CLAUDE.md` se ausentes. | 3 |

Escopo do scanner (`scanner.rs`, `is_in_scope_file`): arquivos de primeiro nível cujo nome começa com `registro_academico`, `cursos_`, `materiais_`, `calendario_`, `notas_de_estudo_`, `futuros_semestres`, `planejamento` ou `engenharia_de_computacao`; qualquer arquivo em `docs/` (exceto `docs/superpowers/`); e `materias/`, `originais/`, `fontes-uva/`, `periodos/`, `cursos/`. **`privado/` e `dados/` não estão no escopo.** Nomes que lembram credenciais (`senha`, `token`, `cookie`, …) são excluídos.

### Canvas, Portal e automação

| Módulo | Responsabilidade e API | Concorrência / erros | Testes |
|---|---|---|---|
| `browser.rs` | `CanvasSession`: `launch`, `status`, `list_courses`, `list_all_courses`, `load_course_inventory`, `read_module_item`, `load_course`, `list_assignment_events`, `capture_canvas_academic`, `open_module_item`, `material_url`, `cookies_for_host`, `current_url`, `open_lyceum`, `lyceum_status`, `refresh_lyceum_catalog`, `close`; `run_browser_smoke`. Chromium **visível** (`with_head`) com perfil próprio, via `chromiumoxide`. | `tokio`, `timeout` de 20 s (inicialização), 5 s (fechamento, saída e `kill`). `BrowserError { Profile, Launch, Page, … }`. | 0 (só o smoke test em `lib.rs`) |
| `canvas.rs` | Parsers puros de HTML do Canvas: `parse_courses`, `parse_course_page`, `parse_module_items`, `group_module_items`, `discover_material_links`, `parse_canvas_page`, `module_item_url`, `content_key`, `reference_key`, `sanitize_source_url`, `CourseListingAccumulator`. | Sem I/O. `CanvasParseError`. | 8 |
| `canvas_api.rs` | Caminhos e tipos da API REST `/api/v1` (`assignments_path`, `discussion_topics_path`, `conversations_path`…), `validate_api_path`, conversão para `GradeItem`, `ForumTopicCapture`, `ConversationCapture`; limites `MAX_COURSES` 40, `MAX_LIST_PAGES` 10, `MAX_TOPICS_PER_COURSE` 60, `MAX_ENTRIES_PER_TOPIC` 2 000, `MAX_CONVERSATIONS` 100. A requisição ocorre em `browser.rs`. | Sem I/O. | 8 |
| `canvas_capture.rs` | Planejamento da captura em lote: `CaptureRun`, `CaptureItem`, `CaptureRunStatus`, `CaptureItemStatus`, `CanvasCaptureLimits::validate`, `build_capture_plan`, `capture_pdf_link`, `resumable_items`, `terminal_run_status`, `recover_interrupted_capture_run`, `confirmed_current_semester`, `is_support_course`. | Sem I/O (a execução está em `src-tauri`, `run_canvas_capture`). | 6 |
| `lyceum.rs` | Portal/Lyceum: `PORTAL_START_URL`, `MAX_PORTAL_PAGES` 50, `navigation_target`, `is_read_only_navigation`, `parse_page`, `login_required`, `persist_capture`, `write_new_file_beneath`, `render_markdown`. Grava arquivos **novos** em `materias/catalogo/capturas/lyceum-<uuid>.{json,md}`. | `PortalState`. | 8 |
| `download.rs` | `validate_material_url` (HTTPS, hosts UVA/Canvas, sem credenciais embutidas nem porta), `download_pdf(url, referer, cookie, validators, limits, cancellation)`; 60 MiB, 4 redirecionamentos manuais, tempo 45 s (aceita 5–120). | `reqwest` sem redirecionamento automático; `watch::Receiver<bool>` para cancelar. `DownloadError`. | 6 |
| `calendar.rs` | `valid_semester`, `assignment_events`, `CalendarEvent`. Só atribuições publicadas, com `due_at` válido e URL do próprio curso. | Sem I/O. | 2 |
| `grading.rs` | `final_grade(&FinalGradeInput) -> FinalGradeResult`: NFp em inteiros (A1×40 + prova×60), aprovação 6,0 (especiais 7,0). Estimativa local. | Sem I/O. Erro `String`. | 8 |

### Conteúdo e estudo

| Módulo | Responsabilidade e API | Testes |
|---|---|---|
| `pdf.rs` | `split_pdf_pages`, `pages_to_markdown` (remove query do URL e escapa HTML), `safe_slug`, `escape_markdown_text`. `PdfError`. | 1 (+ casos em `lib.rs`) |
| `archive.rs` | `import_pdf`, `import_pdf_with_metadata`, `import_canvas_page`, `reuse_existing_material`, `inspect_existing_material`, `reuse_material_for_capture`. Nunca grava o PDF bruto; preserva arquivos existentes (sufixo `-2`). `ArchiveError`. | 6 |
| `lesson.rs` | `build_lesson`, `generate_lesson`, `grade_answers`, `score_answer(s)`: aula e perguntas **determinísticas**, sem IA. | 1 |
| `tutor.rs` | `build_tutor_activity`, `assess_tutor_answer`, `hash_section_markdown`: atividades de estudo e avaliação local. | 11 |
| `study_summaries.rs` | `subject_summary_snapshot` e `generate_subject_summaries`: reúne fontes da matéria (limite `MAX_SUMMARY_SOURCE_BYTES` 2 MiB), exige o hash das fontes confirmado pelo usuário e **envia o texto à API Anthropic**. | 7 |
| `anthropic.rs` | `AnthropicConfig::read_from/read_default`, `AnthropicClient::generate_pair`. Endpoint `https://api.anthropic.com/v1/messages`, modelo padrão `claude-haiku-5-5`, tempo 120 s. `read_default` lê `$HOME/Dev/Projetos/gerenciador-de-conteudo/.env` (caminho fixo no código). `AnthropicError` com mensagens sem segredo. | 8 |

Limitações gerais do núcleo: erros de sincronização e de scanner são `String`, não tipos; `browser.rs` não tem testes unitários; a captura em lote e a importação individual estão em `app/src-tauri/src/lib.rs` (1 888 linhas) e não no núcleo, portanto não são reutilizáveis por um futuro cliente GPUI sem extração.

## `estuda-sync`

<a id="estuda-sync"></a>

- **Estado: código presente, não verificado, não integrado.** O diretório apareceu durante esta revisão e foi crescendo enquanto o documento era escrito. Na última leitura (2026-11-06, por volta de 01:00) `lib.rs` declara 15 módulos e os 15 arquivos existem. **Nenhum comando Cargo foi executado sobre ele**; portanto não se sabe se compila nem se os 112 `#[test]` passam. Nenhum outro crate nem o shell Tauri o chama.
- **Intenção declarada** (`lib.rs`): leitura acadêmica **somente-leitura**, sem UI e sem SQLite; "desconhecido não é zero"; persistência apenas como *proposta* aprovada; só um `PublicRecord` entra no cache/catálogo público.
- **Dependências:** `estuda-domain` (por caminho `../../../crates/estuda-domain`), `scraper`, `serde`, `serde_json`, `sha2`, `thiserror`, `unicode-normalization`, `url`.

| Módulo | Responsabilidade (conforme o cabeçalho do arquivo) | `#[test]` |
|---|---|---:|
| `policy` | `ReadOnlyPolicy` (`uva_default`, `with_host`, `check_request`, `check_action`), `HttpMethod`, `Blocker`, `ElementRole`: toda requisição e todo clique passam por aqui | 7 |
| `source` | `SourceSystem`, `SourceRef`, `strip_query_and_fragment`: proveniência | 2 |
| `link` | `AcademicLink`, `LinkStatus`, `select_active_link`, `split_trajectories`: o estado do vínculo vem da fonte | 6 |
| `verification` | `Verification`, `Coverage`, `check_ava`, `reconcile_field`: ausência de informação não é ausência de entrega | 8 |
| `exam` | `ExamRecord`, `build_records`: data "a confirmar" quando não há confirmação oficial | 6 |
| `p4` | busca do token "P4" sem definir seu significado | 6 |
| `crosscheck` | cruzamento Portal × Canvas: só associação explícita confirma | 5 |
| `changes` | `detect`/`summarize`: diferença entre leituras (ausência não é remoção) | 6 |
| `extract` | `html_to_markdown`, `render_document`: extração determinística de HTML sem IA | 11 |
| `fetch` | `Transport`, `FetchPolicy`, `CancelToken`, `SyncMetrics`: HTTP condicional, TTL, orçamento, retries; o transporte real fica fora do crate | 19 |
| `plan` | `PersistencePlan`, `build_plan`, `Approval`, `PersistenceSink`, `apply`: persistência só com aprovação ligada ao SHA-256 do plano | 11 |
| `privacy` | `scan`, `redact_mechanical`, cadeia `Discovered → OriginVerified → Sanitized → IntegrityChecked → PublicRecord` | 12 |
| `reader` | traits `PortalReader`, `CanvasReader`, `ExamReader`, `CalendarReader`; `map_columns`, `layout_fingerprint` (detecção de mudança de layout) | 3 |
| `store` | `BoundedCache`, `PrivateCache`, `PublicCache`, `DedupIndex`: caches em RAM, privado e público em tipos distintos | 8 |
| `text` | `fold`, `sha256_hex`, `is_word_char` | 2 |

- **Limitações declaradas no próprio código:** os leitores concretos não existem aqui; `reader.rs` registra que os parsers de Sistema de Provas, Zeev e Calendário **não** foram escritos por falta de observação autenticada.
- A aprovação do plano (`plan::Approval`) é o mecanismo de código que espelha a regra do projeto de que mudanças em banco exigem aprovação prévia; ela **não substitui** a aprovação no chat.

## `estuda-gpui`

- **Estado: esqueleto.** Na última leitura (2026-11-06 00:55) `src/main.rs` tinha 27 linhas (31 na leitura seguinte; o arquivo continua mudando): abre uma janela 1280×820 com um `div` de fundo `#0d2d42` e o texto "Estuda UVA", usando `gpui_kit::gpui`. `Cargo.toml` declara `estuda-core` (sem uso no código) e `gpui-kit = "0.9.0"`, e um binário chamado `estuda-uva`. Existe `assets/icons/*.svg`. Até pouco antes dessa leitura o arquivo era `fn main() {}`; o crate estava sendo editado durante a revisão. **Não compilado nem executado por esta revisão.** Detalhes em [migração GPUI](migracao-gpui.md).

## `app/src-tauri` (shell legado)

- **Responsabilidade:** expor `estuda-core` à UI React por 52 comandos Tauri e orquestrar capturas.
- **Entradas/saídas:** JSON serializado (`camelCase`) por `invoke`; eventos `local-sync-updated` e `canvas-capture-updated`.
- **Estado:** `AppState` (ver [eventos e estado](eventos-e-estado.md)).
- **Permissões Tauri:** `capabilities/default.json` concede `core:default` e `dialog:allow-open`. CSP em `tauri.conf.json`: `default-src 'self'` com `connect-src ipc: http://ipc.localhost`.
- **Testes:** nenhum `#[test]` em `src/lib.rs`.
- **Verificação:** `cargo check` deste pacote **não foi executado** nesta revisão.

## Frontend `app/src`

React 19, TypeScript, Vite 8; `app/src/ui/App.tsx` (953 linhas) concentra a navegação (`View = inicio | materias | curriculos | agenda | acervo | estudar | historico | canvas`). Lógica testável fica em `.ts` separados (`studyTypes.ts`, `canvasAcademic.ts`, `canvasCapture.ts`, `lessonSpeech.ts`, `summarySpeech.ts`, `asyncCommand.ts`, `academicCatalog.ts`), cobertos por 40 testes em `app/scripts/*.test.mjs`. Telas: [aplicativo/telas.md](../aplicativo/telas.md).

## Gerador `tools/acervo`

Node 24, sem dependências externas (usa `node:sqlite`). `build.mjs` valida `dados/` e gera `materias/catalogo/*.json`, `build/acervo.sqlite3` e `build/visualizacoes/*.md`. `pdf-para-md.mjs` depende do utilitário `pdftotext`. Ver [README do gerador](../../tools/acervo/README.md).
