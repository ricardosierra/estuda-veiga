# Eventos e estado

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada. Todas as afirmações vêm de `app/src-tauri/src/lib.rs`, `app/crates/estuda-core/src/` e `app/src/ui/App.tsx`.

## Estado do processo (`AppState`)

Definido em `app/src-tauri/src/lib.rs` e registrado com `app.manage(...)` no `setup`.

| Campo | Tipo | O que guarda | Sincronização |
|---|---|---|---|
| `store` | `AcademicStore` | Banco SQLite do app (materiais, capturas, sessões, calendário, configurações) | `std::sync::Mutex<Connection>` interno |
| `sync` | `Arc<SyncEngine>` | Motor de sincronização (outra conexão ao mesmo arquivo) | `Mutex` interno |
| `catalog` | `Arc<CatalogService>` | Índice em memória do acervo (instantâneo + texto por caminho) | `Mutex` interno |
| `watcher` | `SyncWatcher` | Thread de monitoramento de arquivos | `Arc<WatcherState>`, `AtomicBool` |
| `canvas_operation` | `tokio::sync::Mutex<()>` | Serializa operações que tocam a sessão do navegador | exclusão mútua assíncrona |
| `session` | `tokio::sync::Mutex<Option<CanvasSession>>` | Sessão Chromium (Canvas e Lyceum), ou `None` | exclusão mútua assíncrona |
| `active_imports` | `tokio::sync::Mutex<HashMap<String, watch::Sender<bool>>>` | Importações individuais em curso, por `job_id` (UUID) | cancelamento por `watch` |
| `active_captures` | idem | Capturas em lote em curso, por `run_id` | cancelamento por `watch`; no máximo uma captura ativa |

Regras observadas:

- `connect_canvas` e `disconnect_canvas` tomam `canvas_operation` e depois `session`. Chamar `connect_canvas` com sessão existente devolve o status atual, sem abrir outro navegador.
- `start_canvas_capture` e `schedule_canvas_capture` recusam iniciar se `active_captures` não estiver vazio ("Já existe uma captura em andamento").
- `import_material` recusa `job_id` que não seja UUID e recusa job duplicado.
- Comandos de leitura pesada (`sync_local_catalog`, `refresh_local_catalog`, `read_local_document`, `read_local_asset`, `search_local_library`) rodam em `tauri::async_runtime::spawn_blocking`, para não bloquear o executor assíncrono.

## Estado persistido

| O que | Onde | Observação |
|---|---|---|
| Banco do app e do sincronizador | `<acervo>/.estuda-uva.sqlite3` (WAL) | Fora do Git (`.gitignore`). Esquema em [banco privado](../dados/banco-privado.md) |
| Espelhos de sincronização | `<acervo>/.estuda-uva/mirrors/` | Gerados; fora do Git |
| Perfil do Chromium | `<app_data_dir>/chromium-profile` | Separado do navegador pessoal; contém cookies de sessão |
| Caminho do acervo e do Chromium | tabela `settings` (`library_path`, `chromium_executable`) | |
| Limites da captura do Canvas | `AcademicStore::get/save_capture_limits` | Validados por `CanvasCaptureLimits::validate` |

## Estado na interface (React)

`App.tsx` mantém o estado em `useState` (view atual, status do Canvas e do Portal, instantâneo local, relatório de sincronização, conflitos, execução de captura etc.). Não há biblioteca de estado global. `View` assume os valores `inicio`, `materias`, `curriculos`, `agenda`, `acervo`, `estudar`, `historico`, `canvas`.

## Eventos

| Evento | Quem emite | Carga | Quem escuta | Efeito |
|---|---|---|---|---|
| `local-sync-updated` | callback de `SyncWatcher::start` em `run()` (`app_handle.emit`) | `SyncReport` (`scanned`, `created`, `updated`, `unchanged`, `conflicts`, `pendingOperations`, `errors`) | `App.tsx` (`listen`) | Guarda o relatório e executa `refreshLocalLibrary` |
| `canvas-capture-updated` | `emit_capture_update` durante `run_canvas_capture` | `CaptureRun` | `App.tsx` (`listen`) | Atualiza a execução, recarrega os itens e, ao chegar a estado terminal, recarrega materiais e acervo |

Estados terminais tratados na UI: `waiting_for_login`, `cancelled`, `partial`, `completed`, `failed`.

Há também **polling**: `refresh()` roda a cada 3 s (`setInterval` em `App.tsx`) e chama `canvas_status`, `overview`, `list_materials`, `list_calendar_events` e `lyceum_status`. Os eventos são unidirecionais (backend → UI); a UI responde por comandos `invoke`.

## Cancelamento

- Importação: `cancel_import(job_id)` envia `true` no `watch::Sender`; `import_material_inner` e `download_pdf` verificam o receptor entre as etapas (antes de gravar nada).
- Captura em lote: `cancel_canvas_capture(run_id)` faz o mesmo; o estado final registrado é `cancelled`, e a execução pode ser retomada (`resume_canvas_capture`).
- Execuções interrompidas (processo encerrado no meio) são marcadas por `recover_interrupted_capture_run` (`canvas_capture.rs`), chamada em `canvas_capture_status` (`lib.rs`, ~linha 525) quando a execução não está ativa em `active_captures`; a recuperação ocorre sob demanda, quando a UI consulta o status (por exemplo ao abrir a tela); não há varredura de recuperação no `setup`.

## Erros

Os comandos devolvem `Result<_, String>`; erros tipados do núcleo são convertidos por `error_text` (`to_string`). Mensagens do núcleo são em português e, nos módulos de rede e API, evitam incluir URLs com parâmetros ou segredos (testes: `messages_api_errors_are_sanitized` em `anthropic.rs`; sanitização de URL em `canvas::sanitize_source_url` e `lyceum::sanitize_source_url`).

## Limitações

- Estado e eventos dependem do Tauri (`State`, `AppHandle`, `Emitter`). Para o GPUI será preciso substituir; ver [migração](migracao-gpui.md).
- Não há ordem garantida entre `local-sync-updated` e o polling de 3 s; a UI trata ambos de forma idempotente (relê o estado).
- Duas conexões SQLite (`AcademicStore` e `SyncEngine`) para o mesmo arquivo: coerência depende do WAL e de transações curtas; não há teste de carga concorrente.
