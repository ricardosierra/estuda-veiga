# Estuda UVA Anthropic Study Summaries Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add short and long Anthropic-generated study summaries for each active subject, save them locally, and let Sierra listen through the device voice.

**Architecture:** Keep API credentials and requests in a Rust/Tauri backend module; resolve only readable course materials for the active subject and semester, then save complete summary pairs and source hashes in the app's local SQLite store. Add a Resumos tab in the existing subject view and use the Web Speech API for local playback.

**Tech Stack:** Rust, Tauri 2, reqwest, SQLite via rusqlite, serde/serde_json, sha2, React, TypeScript, Node's built-in test runner.

**Spec:** `docs/superpowers/specs/2026-10-14-estuda-uva-resumos-anthropic-design.md`

## Global Constraints

- A geração acontece somente quando a usuária acionar **Gerar resumos**.
- Resumo curto (alvo de aproximadamente 2 minutos de leitura) e resumo longo (alvo de aproximadamente 8 minutos).
- Ler `ANTHROPIC_API_KEY` e `ANTHROPIC_MODEL` sob demanda do arquivo `~/Dev/Projetos/gerenciador-de-conteudo/.env`, sem alterar esse arquivo, copiar o segredo para outro local, imprimir o valor ou gravá-lo no SQLite/Markdown/JSON/Git.
- A fonte da geração é somente o material didático legível associado à matrícula ativa e ao semestre da tela. Não incluir notas, frequência, registros oficiais, credenciais ou cookies.
- Persistir os resumos e metadados numa tabela própria do banco SQLite local do app, sem editar os originais ou tratar os resumos como registros acadêmicos oficiais.
- Não registrar chave, cabeçalhos de autenticação, texto integral dos materiais, requisição ou resposta da API nos logs.
- Configuração ausente ou erro da API não deve impedir o restante do Estuda UVA nem a abertura das aulas existentes.
- Usar a Web Speech API no dispositivo com idioma inicial `pt-BR`; não criar nem enviar arquivos de áudio.

## Review Focus

- Missing, empty, quoted, or malformed `.env` entries must yield a redacted configuration error and leave the rest of the app available.
- A subject with no readable source, a different-semester source, or a source path outside the configured library must not send a request.
- Source text may contain instructions; the summary prompt must treat all source text as study material and require grounded explanations and source references.
- HTTP 401/403, 429, 5xx, timeout, malformed response, or incomplete short/long output must preserve the previous saved summary and return a sanitized error.
- A WebView without a Portuguese voice, a rapid subject change, or unmount during playback must report/stop speech cleanly without changing lesson progress.

---

### Task 1: Persist generated summaries locally

**Files:**
- Modify: `app/src-tauri/src/storage.rs`

**Interfaces:**
- Produces `StudySummary` with `id`, `course_id` (the stable active-enrollment id), `course_title`, `semester`, `source_hash`, `source_manifest_json`, `short_text`, `long_text`, `model`, `input_tokens`, `output_tokens`, and `created_at`.
- Produces `AcademicStore::save_study_summary(&StudySummary) -> Result<(), StorageError>`.
- Produces `AcademicStore::latest_study_summary(course_id: &str, semester: &str) -> Result<Option<StudySummary>, StorageError>`.
- Keep prior rows when a newer source hash is summarized so the UI can display the newest result as stale without deleting it.

- [x] **Step 1: Write the migration and round-trip test**

The shared checkout contains an uncommitted schema-v5 lesson-interaction migration and a schema-v7 Canvas-capture migration. Preserve both. The summary table is introduced at version 6; opening a version-4 or version-5 database now runs through version 7. Add `study_summary_schema_migrates_v4_to_v7_and_preserves_prior_generations`, using a temporary database with `user_version=4`. Open it through `AcademicStore::open`, save two summaries for the same subject and semester with different source hashes, and assert final schema version 7, both rows retained, and the newest row returned by `latest_study_summary`. Also cover upgrading an existing schema-v5 database to v7.

```rust
assert_eq!(schema_version, 7);
assert_eq!(stored_row_count, 2);
assert_eq!(latest.source_hash, "source-b");
```

- [x] **Step 2: Run the focused storage test and confirm it fails**

Run from `app/src-tauri`: `cargo test --lib study_summary_schema_migrates_v4_to_v7_and_preserves_prior_generations`  
Expected: FAIL because the schema and methods do not exist yet.

- [x] **Step 3: Add schema version 6 and the summary persistence methods**

Add a `study_summaries` table with a primary key, subject/semester identifiers, source hash and source manifest, short/long text, model, token counts, and creation timestamp. Introduce this table in schema migration v6, then preserve and run the concurrent v7 Canvas-capture migration. Add the two methods from the Interfaces block; never include API credentials in this table.

- [x] **Step 4: Rerun the focused storage test**

Run from `app/src-tauri`: `cargo test --lib study_summary_schema_migrates_v4_to_v7_and_preserves_prior_generations`  
Expected: PASS with version 4 and version 5 upgraded to 7, the summary table introduced at version 6, and both generations retained.

- [ ] **Step 5: Review and commit only this task's file**

Inspect `git diff -- app/src-tauri/src/storage.rs`, then stage only that file and commit as `feat: persist subject study summaries`.

### Task 2: Add the Anthropic Messages API client

**Files:**
- Create: `app/src-tauri/src/anthropic.rs`
- Modify: `app/src-tauri/src/lib.rs` (module registration)

**Interfaces:**
- Produces `AnthropicConfig::read_from(path: &Path) -> Result<AnthropicConfig, AnthropicError>`, reading only `ANTHROPIC_API_KEY` and `ANTHROPIC_MODEL`. Keep the key private, omit `Debug`, and return errors that never contain the value.
- Produces `SummaryPair { short_text, long_text, input_tokens, output_tokens, model }`.
- Produces `SummarySource { label, markdown }` for the trusted backend source collector to pass to the API client.
- Produces `AnthropicClient::from_config(config)` for production and a private endpoint-injection seam for local HTTP tests; production requests always target Anthropic.
- Produces async `AnthropicClient::generate_pair(subject: &str, sources: &[SummarySource]) -> Result<SummaryPair, AnthropicError>`.
- Read the default config from `home_dir()/Dev/Projetos/gerenciador-de-conteudo/.env`; default the model to `claude-haiku-5-5` only when `ANTHROPIC_MODEL` is absent or empty.
- Send a single Messages API request to `https://api.anthropic.com/v1/messages` with `x-api-key`, `anthropic-version`, and JSON content headers; use one response with separate `## Resumo curto` and `## Resumo longo` sections. Reject missing sections and `max_tokens` truncation.

- [x] **Step 1: Write configuration and response-parser tests**

Add tests named `env_parser_reads_only_key_and_model_with_quotes_and_crlf`, `missing_or_empty_api_key_returns_redacted_error`, `malformed_env_quotes_return_redacted_error`, `parser_rejects_response_without_both_summary_sections`, and `parser_rejects_max_tokens_stop_reason`. Use synthetic test credentials only. Assert the parsed pair contains both sections, the configured model, and token counts; assert error strings do not contain the synthetic credential.

```rust
assert_eq!(config.model(), "claude-haiku-5-5");
assert!(error.to_string().contains("ANTHROPIC_API_KEY"));
assert!(!error.to_string().contains("synthetic-test-key"));
assert!(pair.short_text.contains("Resumo curto"));
assert!(pair.long_text.contains("Resumo longo"));
```

- [ ] **Step 2: Run the focused client tests and confirm they fail**

Run from `app/src-tauri`: `cargo test --lib anthropic::tests`  
Expected: FAIL because the module and parser are not implemented.

- [x] **Step 3: Implement config parsing, request construction, and sanitized errors**

Read the selected `.env` file as data; parse only the two named assignments and support ordinary unquoted or quoted values. Use the existing `reqwest` dependency and Rust-only request handling with a bounded timeout. Do not log request/response bodies or authentication headers. Map HTTP status, network, and timeout failures to messages that do not include server response text.

- [x] **Step 4: Add a local HTTP mock test for the Messages API contract**

Add `messages_api_sends_auth_header_and_returns_complete_pair` using a local `TcpListener` mock through the private endpoint-injection seam, not Anthropic's live endpoint. Assert the request uses `POST /v1/messages`, the configured model, `x-api-key`, `anthropic-version`, and one user message containing the course source labels; return a fixture response and assert both summaries and token counts are parsed. Add cases asserting 401/403, 429, 5xx, timeout, and malformed provider responses return sanitized errors and never expose a mock error body.

```rust
assert_eq!(request.method(), "POST");
assert_eq!(request.path(), "/v1/messages");
assert_eq!(request.header("x-api-key"), Some("synthetic-test-key"));
assert_eq!(pair.input_tokens, 120);
assert_eq!(pair.output_tokens, 900);
assert!(!error.to_string().contains("mock response body"));
```

- [x] **Step 5: Run the focused Anthropic client tests**

Run from `app/src-tauri`: `cargo test --lib anthropic::tests`  
Expected: PASS without reading the real key or making a billed request.

- [ ] **Step 6: Review and commit only this task's files**

Inspect the staged diff for key leakage, then stage only `app/src-tauri/src/anthropic.rs` and `app/src-tauri/src/lib.rs`; commit as `feat: add Anthropic summary client`.

### Task 3: Resolve subject sources and expose Tauri commands

**Files:**
- Create: `app/src-tauri/src/study_summaries.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src-tauri/src/storage.rs` only if the task 1 interface needs a narrowly scoped correction

**Interfaces:**
- Produces serializable `SummarySourcePreview { relative_path, title, format, size_bytes }` and `SubjectSummarySnapshot { sources, current_source_hash, latest_summary }`.
- Produces `collect_subject_sources(catalog: &CatalogService, store: &AcademicStore, course_title: &str, semester: &str) -> Result<Vec<SummarySource>, String>` using the source type defined in Task 2.
- Adds Tauri command `get_subject_summary_snapshot(course_id, course_title, semester)` where `course_id` is `ActiveEnrollment.id`; it refreshes the catalog before returning source preview, current hash, and the most recent saved pair.
- Adds Tauri command `generate_subject_summaries(course_id, course_title, semester, confirmed_source_hash)` where `course_id` is `ActiveEnrollment.id`; it refreshes and re-collects sources, rejects a hash mismatch before calling Anthropic, and persists only a complete response.

- [x] **Step 1: Write source-selection and hashing tests**

Add tests named `collects_only_readable_material_for_matching_course_and_semester`, `rejects_imported_material_outside_library_root`, `source_hash_is_order_independent_and_content_sensitive`, `changed_source_hash_requires_new_confirmation`, `no_sources_prevents_api_generation`, and `failed_generation_keeps_previous_summary`. Fixtures must include another semester, an official record named inside a recognized course folder, and an unreadable file; assert none appear in the outbound source list even when the folder-based category says `material acadêmico`. Use a fake generator/call counter for the no-source, hash-mismatch, and failure cases so they prove Anthropic is never invoked for invalid input and stored text is retained after a generation failure.

```rust
assert_eq!(sources.iter().map(|item| &item.label).collect::<Vec<_>>(), expected_paths);
assert!(collect_result.unwrap_err().contains("fora da pasta"));
assert_eq!(hash_a, hash_with_reordered_sources);
assert_ne!(hash_a, hash_after_source_edit);
assert_eq!(mock_request_count, 0);
```

- [ ] **Step 2: Run the focused source tests and confirm they fail**

Run from `app/src-tauri`: `cargo test --lib study_summaries::tests`  
Expected: FAIL because the source resolver and commands do not exist.

- [x] **Step 3: Implement source resolution and a stable content hash**

Refresh the catalog in a blocking worker before every preview and generation so newly added or edited files are reflected. Use `CatalogService::snapshot` and `read_document`. Match the normalized course title and exact semester, require searchable text categorized as `material acadêmico`, and include imported material text only when its stored path resolves beneath `AcademicStore::library_path`. Read the imported `path` source, not `lesson_path`; canonicalize and enforce root containment, deduplicate by relative path, then sort labels/paths before hashing with SHA-256. Keep only relative path/title/content hash in the persisted source manifest.

- [x] **Step 4: Add the snapshot and generation Tauri commands**

Implement the two command signatures in the Interfaces block, register them in `tauri::generate_handler!`, and call storage only after the API response contains both summaries. The UI passes the preview's `current_source_hash` as confirmation; if a fresh collection differs, reject before the API call and return an error that makes the UI reload the source preview for a new confirmation. If there are no readable sources, return a clear local error without creating an HTTP request. Persist source manifest, source hash, model, and token counts with both texts. Order `latest_study_summary` by `created_at DESC, rowid DESC` so same-second generations still return the last inserted row.

- [x] **Step 5: Run source tests and relevant backend tests**

Run from `app/src-tauri`: `cargo test --lib study_summaries::tests` and `cargo test --lib storage::tests`  
Expected: PASS; prior stored generations remain available after a failed request or source change.

- [ ] **Step 6: Review and commit only this task's files**

Inspect the diff for path containment and academic-data filtering, then stage only `app/src-tauri/src/study_summaries.rs`, `app/src-tauri/src/lib.rs`, and any specifically corrected `storage.rs`; commit as `feat: generate summaries from local subject materials`.

### Task 4: Add the Resumos view and local speech controls

**Files:**
- Create: `app/src/ui/StudySummaryPanel.tsx`
- Create: `app/src/ui/summarySpeech.ts`
- Create: `app/scripts/study-summary.test.mjs`
- Modify: `app/src/ui/SubjectDetail.tsx`
- Modify: `app/src/ui/studyTypes.ts`
- Modify: `app/src/ui/styles.css`

**Interfaces:**
- `StudySummaryPanel` consumes `ActiveEnrollment`; invokes `get_subject_summary_snapshot` and `generate_subject_summaries`. The backend snapshot supplies the authoritative source list and source hash.
- `summarySpeech.ts` exports `speakSummary(text: string): boolean`, `pauseSummary()`, `resumeSummary()`, and `stopSummary()`; it returns `false` when speech synthesis is unsupported and does not throw.
- `StudySummaryPanel` shows the exact source list and asks for confirmation before the generation command; it passes the shown source hash with that confirmation, reloads the preview if it changed, and displays separate short/long summaries, model/date/source metadata, and stale state by comparing source hashes.

- [x] **Step 1: Write speech helper tests**

Add Node tests `speaks_in_portuguese_and_stops_current_utterance`, `pause_and_resume_delegate_to_speech_synthesis`, and `unsupported_speech_synthesis_returns_unavailable`. Mock `speechSynthesis` and `SpeechSynthesisUtterance`; assert `lang === "pt-BR"`, stop calls `cancel()`, and no browser globals are needed merely to import the helper.

```js
assert.equal(speakSummary("conceito"), true);
assert.equal(utterance.lang, "pt-BR");
assert.equal(speech.cancelCalls, 1);
assert.equal(speakSummary("sem navegador"), false);
```

- [x] **Step 2: Run the focused UI helper tests and confirm they fail**

Run from `app`: `node --experimental-strip-types --test scripts/study-summary.test.mjs`  
Expected: FAIL because the speech helper is not implemented.

- [x] **Step 3: Implement the local speech helper**

Implement the exported functions in `summarySpeech.ts`; do not persist playback as a lesson session or academic progress. Cancel speech when the panel unmounts or its subject changes.

- [x] **Step 4: Add the Resumos tab and panel**

Add `resumos` to `SubjectDetail` tabs and render `StudySummaryPanel` for the active enrollment. Show the exact source names and active semester before generation, state explicitly that these texts will be sent to Anthropic and may use credits, and only invoke the generation command after confirmation with the preview hash. Display sanitized errors, loading state, saved summaries, stale labels, and separate listen/pause/resume/stop controls. Stop current speech on subject/semester change and on unmount.

- [x] **Step 5: Run UI tests and TypeScript/Vite build**

Run from `app`: `node --experimental-strip-types --test scripts/study-summary.test.mjs` and `npm run build`  
Expected: all helper tests PASS and the React/TypeScript/Vite build succeeds.

- [ ] **Step 6: Review and commit only this task's files**

Inspect the component diff for confirmation-before-send and playback cleanup, stage only the six files listed in this task, and commit as `feat: add listenable subject summaries`.

### Task 5: Verify the integrated flow

**Files:**
- Modify only implementation/test files from Tasks 1–4 if verification exposes a defect.

**Interfaces:**
- The UI snapshot and generation commands use the same `course_id`, `course_title`, and `semester` values from `ActiveEnrollment`.
- The API client is verified through a local mock only; no test bypasses the UI confirmation or sends academic materials to an external service.

- [ ] **Step 1: Run focused format and regression checks**

Run from `app/src-tauri`: `cargo fmt --check` and `cargo test --lib`.  
Run from `app`: `npm test` and `npm run build`.  
Expected: all commands exit 0; existing lesson, catalog, and storage tests remain green.

- [ ] **Step 2: Build the desktop app**

Run from `app`: `npm run tauri -- build`.  
Expected: Tauri builds with the new commands and the generated frontend bundle.

- [ ] **Step 3: Perform the local UI acceptance pass**

Open a subject with current-semester source material, confirm the preview omits other semesters and official-record categories, verify cancellation sends no request, then use the local mock-backed test coverage for success/error handling. Reopen the subject and confirm the saved pair is available offline, marked stale after source content changes, and prior text remains after a failed regeneration. Test play/pause/resume/stop and a WebView without speech synthesis.

- [ ] **Step 4: Review working tree and report evidence boundaries**

Confirm the `.env` and academic originals were not modified; inspect `git status` and the final implementation diff. Report source tests, full app tests, build, and desktop playback separately. Do not report a live Anthropic call unless the user explicitly initiates the generation action in the UI.

- [ ] **Step 5: Commit only a focused integration fix, if needed**

If verification requires a narrowly scoped compile/test correction in shared source, record the ruling in the SDD ledger, preserve its existing behavior, and include a regression test. Stage only feature-owned hunks and never include the existing academic catalog/material changes or unrelated planning files. Do not make an extra commit when overlapping shared work prevents a safe feature-only stage.
