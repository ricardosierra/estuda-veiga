# Estuda UVA: captura automática de aulas do Canvas — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Estuda UVA capture published lesson pages and accessible PDFs across all current and past courses, plus future courses already visible and published in the authenticated Canvas account, with resumable local progress.

**Architecture:** Keep Canvas access in the existing visible Chromium session controlled by Rust/CDP. Add safe inventory/page parsers, local SQLite capture jobs and item states, conflict-safe text persistence, one sequential capture runner, and a React progress panel. The runner uses read-only navigation, saves Canvas pages and PDF text as Markdown, emits sanitized progress, and never marks attendance or creates study sessions merely for importing content.

**Tech Stack:** Tauri 2, Rust, `chromiumoxide`/CDP, `scraper`, `reqwest`, `pdf-extract`, rusqlite, React, TypeScript, Node's built-in test runner.

**Spec:** `docs/superpowers/specs/2026-10-14-estuda-uva-varredura-automatica-canvas-design.md`

## Global Constraints

- Reuse the visible Chromium profile and manual login; do not add Playwright or a Node runtime to the installed app.
- Traverse only courses and published content exposed by the authenticated Canvas account; IDs remain dynamic.
- Use read-only Canvas navigation; never mark presence, module completion, grades, assignments, quizzes, forums, or enrollment.
- Process one browser item at a time; cancellation, session expiry, and per-item failures keep confirmed results and support resumption.
- Persist no passwords, cookies, tokens, signed URLs, query strings, or fragments in SQLite, Markdown, logs, or events; sanitize error strings and event payloads too.
- Validate each PDF URL and redirect against the existing allowlist, check the response type and `%PDF-` signature, extract bytes in memory only, and discard them after Markdown extraction.
- Keep searchable Markdown, and preserve every existing source file and manual edit.
- Keep capture-run state local in SQLite. Keep Markdown/JSON/SQLite content projections synchronized through the existing sync path.
- Preserve the shared dirty checkout. Planning baseline already has `CURSOS_2026.2.md`, `MATERIAIS_2026.2.md`, and `materias/fontes-uva/2026.2/README.md` modified, plus existing untracked academic archives. Record a fresh status/diff before implementation, preserve all those changes, and do not reset, clean, stage, commit, push, or install over `/Applications`. Use one authenticated sample course for validation; the user starts a full capture from the completed app.
- Run focused Rust/frontend checks and report source, build, and authenticated-runtime evidence separately.

## Review Focus

- A next-course link is external, malformed, or repeats: stop pagination safely and retain a partial-state error. Test: `course_pagination_rejects_external_and_repeated_links` (Task 1).
- An item is hidden, unpublished, locked, or an assessment: skip it without opening and retain a safe reason when the inventory exposes it. Test: `inventory_excludes_hidden_unpublished_and_assessment_items` (Task 1).
- Canvas returns an item link whose query contains a short-lived signature: persist only a stable sanitized URL/key. Tests: `content_key_drops_query_and_fragment` and `capture_item_source_url_drops_query_and_fragment_on_persist` (Tasks 1–2).
- The same PDF/page appears under multiple modules or a past/current course: dedupe by Canvas source ID where present, otherwise canonical source URL plus kind, and retain all associations. Test: `capture_plan_deduplicates_content_and_keeps_module_references` (Tasks 1 and 4).
- The user edits a Markdown file before a later capture: preserve it and mark the source for review instead of replacing it. Test: `capture_does_not_replace_edited_local_material` (Task 3).
- Authentication expires or cancellation arrives during a download: stop before the next item, keep completed items, and allow resume. Test: `capture_state_pauses_or_cancels_without_losing_completed_items` (Tasks 2 and 4).
- A response is not a PDF or has invalid signature, a redirect crosses to an unapproved host, or cancellation arrives: reject it without persisting bytes. Tests: `pdf_download_rejects_invalid_type_and_signature`, `pdf_redirect_revalidates_each_host`, and `cancelled_conditional_download_starts_no_request` (Task 3).

---

### Task 1: Build a safe, complete Canvas content inventory

**Files:**
- Modify: `app/src-tauri/src/canvas.rs`
- Modify: `app/src-tauri/src/browser.rs`
- Test: unit tests and sanitized fixtures in `app/src-tauri/src/canvas.rs`

**Interfaces:**
- `CanvasCoursePage { courses: Vec<Course>, next_href: Option<String> }` represents one course-list page.
- `CanvasCourseListing { courses: Vec<Course>, pages_traversed: usize, complete: bool, warning: Option<String> }` makes a configured page limit visible to the caller.
- `CanvasCourseInventory { modules: Vec<ModuleGroup>, complete: bool, warning: Option<String> }` records whether every visible, published module was enumerated.
- `CanvasModuleItemContent { canvas_item_id: Option<String>, title: String, source_url: String, markdown: Option<String>, pdf_links: Vec<MaterialLink>, unsupported_reason: Option<String> }` represents one inspected module item.
- `content_key(canvas_id: Option<&str>, source_url: &str, source_kind: &str) -> Option<String>` identifies shared content across courses using the Canvas source ID when present, otherwise canonical origin/path plus kind; `reference_key(course_id: &str, module_title: &str, item_href: &str) -> Option<String>` identifies one course/module occurrence with query and fragment removed.
- `parse_course_page(html: &str, page_url: &str) -> CanvasCoursePage` returns courses exposed by the authenticated listing (current, past, and future already visible/published) and a same-origin pagination link.
- `parse_canvas_page(html: &str, page_url: &str) -> Result<CanvasModuleItemContent, CanvasParseError>` extracts safe textual structure and embedded PDF references, dropping scripts, active attributes, and unsafe links.
- Preserve existing `CanvasSession::list_courses` behavior for manual UI callers. Add `CanvasSession::list_all_courses(&self, max_pages: usize) -> Result<CanvasCourseListing, BrowserError>` for batch traversal; enforce the validated configured limit (maximum 100), deduplicate course IDs, and return `complete=false` with a warning when a next page remains at the cap.
- `CanvasSession::load_course_inventory(&self, course_id: &str, max_items: usize) -> Result<CanvasCourseInventory, BrowserError>` returns visible, published, non-assessment modules/items in Canvas order, and a partial warning if the configured item limit truncates inventory. Items marked hidden, unpublished, or locked are excluded without opening.
- `CanvasSession::read_module_item(&mut self, course_id: &str, href: &str) -> Result<CanvasModuleItemContent, BrowserError>` accepts only a same-origin Canvas course/module/page URL, checks authentication before and after navigation, and returns sanitized content/links or an explicit unsupported reason. Raw signed URLs may exist only in memory for the immediate request; persisted links and keys are sanitized.

- [x] **Step 1: Add parser/navigation tests** for `course_pagination_rejects_external_and_repeated_links`, `course_listing_reports_configured_page_limit_as_partial`, `parse_module_items_preserves_visible_order_and_excludes_assessments`, `inventory_excludes_hidden_unpublished_and_assessment_items`, `parse_canvas_page_drops_scripts_and_preserves_headings_lists_and_safe_links`, and `content_key_drops_query_and_fragment`. Fixtures contain synthetic course IDs, URLs, and lesson text only.
- [x] **Step 2: Run the focused Rust tests and confirm the new parser cases fail** because the types/functions or expected safety behavior do not exist yet.
- [x] **Step 3: Implement the course-page, module-item, and Canvas-page parsers** in `canvas.rs`; parse publication/lock indicators conservatively, exclude assessments, normalize content keys from source ID or source kind plus URL origin/path, and remove query and fragment from keys/URLs.
- [x] **Step 4: Implement bounded same-origin course/module inventory and `CanvasSession::read_module_item`** in `browser.rs`. Preserve `list_courses`, `load_course`, and `open_module_item` behavior for existing manual callers.
- [x] **Step 5: Run the focused Canvas tests** with `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib canvas::tests`; expect all parser cases to pass and hidden, locked, unpublished, and assessment routes to remain unopened. Observed: 8 passed, 0 failed; the full crate test target compiled with unrelated tutor/summary changes present.

### Task 2: Persist resumable capture runs and safe source metadata

**Files:**
- Create: `app/src-tauri/src/canvas_capture.rs`
- Modify: `app/src-tauri/src/lib.rs` (module declaration and state types only in this task)
- Modify: `app/src-tauri/src/storage.rs`
- Test: `storage.rs` migration and capture-state tests

**Interfaces:**
- `CaptureRunStatus`: `running | waiting_for_login | cancelled | partial | completed | failed`.
- `CaptureItemStatus`: `pending | running | captured | reused | ignored | failed | needs_review`.
- `CaptureCounts { courses_total, courses_done, items_total, captured, reused, ignored, failed, needs_review, pending }`.
- `CaptureRun { id, scope, limits_snapshot, status, counts, current_course_id, current_course_title, current_module_title, current_item_title, warning, started_at, updated_at, finished_at }` records the scope and validated limits used by that run.
- `CanvasCaptureLimits { max_course_pages, max_courses, max_items, max_pdf_bytes, max_redirects, item_timeout_seconds }` defaults to 100 pages, 200 courses, 10,000 items, 60 MiB, 4 redirects, and 45 seconds; validation bounds are 1–100 pages, 1–500 courses, 1–20,000 items, 1–60 MiB, 0–4 redirects, and 5–120 seconds.
- `CaptureItem { reference_key, content_key, canvas_item_id, course_id, course_title, module_title, title, source_kind, source_url, status, retryable, attempts, material_id, message, updated_at }` records each course/module occurrence while sharing one content key for duplicate sources. `source_url` is sanitized before it reaches storage; unsupported ignored items are not retryable.
- `MaterialAssociation { material_id, course_id, course_title, module_title, item_title, semester }` links a shared text file to every course/module that uses it.
- `AcademicStore::create_capture_run`, `get_capture_run`, `latest_incomplete_capture`, `update_capture_run`, `upsert_capture_item`, `list_capture_items(run_id, offset, limit)`, `save_material_association`, `get_capture_limits`, and `save_capture_limits` provide transactional persistence and bounded reads. Preserve legacy duplicate rows during migration; dedupe new imports by source key plus text hash in the serialized writer path, and do not add a uniqueness constraint that would require merging old rows.
- `ImportedMaterial` gains stable `source_key`, `source_kind` (`pdf` or `canvas_page`), optional `source_etag`/`source_last_modified`, `text_sha256`, and a list of `MaterialAssociation` values; migration derives each legacy `source_key` from normalized origin/source kind and adds its existing course/module as an association.

- [ ] **Step 1: Add tests** named `capture_schema_migrates_v6_to_v7_preserves_materials_and_adds_capture_tables`, `capture_run_counts_follow_item_state_changes`, `capture_latest_incomplete_run_is_resumable`, `capture_limits_default_and_reject_values_outside_bounds`, `repeated_content_adds_a_course_association_without_duplicate_material`, and `capture_item_source_url_drops_query_and_fragment_on_persist`.
- [ ] **Step 2: Run only those tests and confirm they fail** on the current schema v6 and missing capture model/store methods. Keep the existing v4→v5 tutor-interaction and v5→v6 study-summary migrations intact.
- [ ] **Step 3: Add the capture DTOs and enums** in `canvas_capture.rs`, using camelCase serialization for struct fields and snake_case enum variants.
- [ ] **Step 4: Upgrade SQLite schema to v7** in `storage.rs`; preserve the existing v4→v5 and v5→v6 migrations, then add `source_key`, `source_kind`, nullable `source_etag`/`source_last_modified`/`text_sha256`, `material_associations`, `capture_runs`, and `capture_items`. Backfill canonical global source keys and one association per existing material without deleting or merging existing rows. Keep URLs and error text sanitized before writes and update all material row readers/writers.
- [ ] **Step 5: Implement atomic run/item upserts and derived counters**; `capture_items` is unique by `(run_id, reference_key)`, and a repeated upsert must not create duplicate rows.
- [ ] **Step 6: Implement stored capture-limit defaults and validation**; reject values outside the exact bounds in `CanvasCaptureLimits` without replacing the last valid settings. Snapshot the accepted settings into each run so a later preferences change cannot silently alter its limits.
- [ ] **Step 7: Run the focused storage tests** with `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib storage::tests`; expect legacy v4 fixture rows and current v6 material rows to remain unchanged and the capture tables to be queryable at schema v7.

### Task 3: Save Canvas pages and changed PDFs without overwriting local work

**Files:**
- Modify: `app/src-tauri/src/archive.rs`
- Modify: `app/src-tauri/src/download.rs`
- Modify: `app/src-tauri/src/storage.rs`
- Test: unit tests in `archive.rs` and `download.rs`

**Interfaces:**
- `ArchiveImportStatus`: `captured | reused | needs_review`; `ArchiveImportOutcome { bundle: ImportedBundle, status: ArchiveImportStatus }` keeps archive code independent of the capture-job state machine.
- `archive::import_canvas_page(store, course, module_title, title, source_url, markdown) -> Result<ArchiveImportOutcome, ArchiveError>` writes one safe `.md` material and indexes it with `source_kind=canvas_page`.
- `archive::import_pdf(store, course, module_title, title, semester, source_url, source_etag, source_last_modified, pdf_bytes, pages) -> Result<ArchiveImportOutcome, ArchiveError>` records per-page text and source validators; bytes remain in memory and are dropped after extraction.
- `archive::inspect_existing_material(store, source_key) -> Result<Option<ImportedMaterial>, ArchiveError>` returns the latest indexed version across all courses, only when its local path is contained beneath the configured library root.
- `download::download_pdf(raw_url, referer, cookie, validators, limits, cancellation) -> Result<DownloadOutcome, DownloadError>` retains HTTPS/allowlist/redirect/cancellation protections, revalidates every redirect, applies configured byte/redirect/timeout limits, and never writes PDF bytes or signed URLs to disk.
- `DownloadValidators { host, etag, last_modified }` and `DownloadOutcome::{NotModified, Downloaded { bytes, final_url, etag, last_modified }}` represent conditional response metadata; validator strings are bounded and validated header text only.
- `archive::import_pdf(...)` records the PDF source kind and extracted Markdown hash; the raw PDF remains in memory and is dropped after conversion.

- [ ] **Step 1: Add tests** named `canvas_page_import_writes_searchable_markdown_and_source_metadata`, `unchanged_material_reuses_indexed_markdown`, `changed_source_creates_a_reviewable_version_without_overwrite`, `capture_does_not_replace_edited_local_material`, and `pdf_without_selectable_text_is_ignored_with_safe_reason`.
- [ ] **Step 2: Run these tests and confirm they fail** on existing PDF-only archive import and missing source-kind/revision/integrity metadata.
- [ ] **Step 3: Implement `import_canvas_page`** with sanitized source URL, safe Markdown escaping, stable source key, content hash, course/module metadata, and collision-safe file creation.
- [ ] **Step 4: Make the source key global** from canonical source URL and source kind; identify a content version from source key plus source-content hash. Reuse one stored text file across courses and add each course/module occurrence through `material_associations`.
- [ ] **Step 5: Update PDF/page imports and reuse** so unchanged sources are reused, changed remote content receives a separate version, and edited local Markdown is preserved with `needs_review` rather than overwritten. Compare the local Markdown hash even after a conditional `304`; a matching remote validator never makes an edited local file safe to replace.
- [ ] **Step 6: Add conditional retrieval** to `download_pdf(raw_url, referer, cookie, validators, limits, cancellation) -> Result<DownloadOutcome, DownloadError>`. Send validators only to the matching validated host; a redirect to a different host receives neither the previous host's validators nor its cookies. Validate content type and PDF signature before returning bytes, with bytes held in memory only.
- [ ] **Step 7: Add download tests** `conditional_get_returns_not_modified_without_pdf_bytes`, `conditional_headers_are_not_forwarded_across_hosts`, `pdf_download_rejects_invalid_type_and_signature`, `pdf_redirect_revalidates_each_host`, and `cancelled_conditional_download_starts_no_request`.
- [ ] **Step 8: Run the focused archive/download tests** with `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib archive::tests` and `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib download::tests`; verify no test or code path writes PDF bytes to the archive, and an unextractable PDF is recorded as unsupported with a sanitized reason.

### Task 4: Coordinate sequential capture, progress, cancellation, and resume

**Files:**
- Create or extend: `app/src-tauri/src/canvas_capture.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Test: pure state-transition tests in `canvas_capture.rs`

**Interfaces:**
- `start_canvas_capture(app: AppHandle, state: State<'_, AppState>) -> Result<CaptureRun, String>` creates a run and starts the sequential worker; it rejects unauthenticated sessions and concurrent capture runs.
- `resume_canvas_capture(app: AppHandle, run_id: String, state: State<'_, AppState>) -> Result<CaptureRun, String>` re-discovers the current course/module inventory and skips item keys already confirmed in that run.
- `cancel_canvas_capture(run_id: String, state: State<'_, AppState>) -> Result<(), String>` signals the worker through Tokio `watch` cancellation.
- `canvas_capture_status(run_id: Option<String>, state: State<'_, AppState>) -> Result<Option<CaptureRun>, String>` returns the requested/latest run.
- `canvas_capture_items(run_id: String, offset: usize, limit: usize, state: State<'_, AppState>) -> Result<Vec<CaptureItem>, String>` returns bounded per-item results.
- `retry_canvas_capture_item(app: AppHandle, run_id: String, reference_key: String, state: State<'_, AppState>) -> Result<CaptureRun, String>` schedules one failed/ignored item again without resetting completed items.
- `get_canvas_capture_limits(state: State<'_, AppState>) -> Result<CanvasCaptureLimits, String>` returns the saved settings/defaults.
- `set_canvas_capture_limits(limits: CanvasCaptureLimits, state: State<'_, AppState>) -> Result<CanvasCaptureLimits, String>` validates then persists local settings.
- `run_canvas_capture(app: AppHandle, run_id: String, cancellation: watch::Receiver<bool>)` processes course → module → item in source order, applies the run's limits snapshot, persists every terminal item state, and emits `canvas-capture-updated` with a sanitized `CaptureRun` after each state change.

- [ ] **Step 1: Add pure state-machine tests** named `capture_plan_deduplicates_content_and_keeps_module_references`, `capture_state_pauses_or_cancels_without_losing_completed_items`, and `capture_run_resumes_only_pending_or_retryable_items`.
- [ ] **Step 2: Run those tests and confirm they fail** before implementing the state-transition and dedupe helpers.
- [ ] **Step 3: Implement a deterministic capture plan** from `CanvasCourseListing`/`CanvasCourseInventory`; include visible current/past and visible published future courses, carry every `complete=false` warning into the run, and never enqueue hidden/unpublished/locked items, quizzes, submissions, discussions, grades, attendance, profiles, or non-text media.
- [ ] **Step 4: Implement the worker** to call `read_module_item`, persist Canvas page Markdown and each validated PDF through Task 3, add course/module associations for shared content, record per-source outcomes, and continue after an item-level error. Direct PDF module items and PDF links discovered on a lesson page become separate source items with their own stable reference keys.
- [ ] **Step 5: Add authentication/cancellation checkpoints** before every navigation, request, extraction result, and persistence operation, and re-check Canvas session state after authentication-related download errors. Session expiry sets `waiting_for_login`; cancellation sets `cancelled` without committing an incomplete item; item failures or bounded-limit truncation set `partial`; a fatal run-level error sets `failed`; all outcomes retain completed item rows.
- [ ] **Step 6: Add `active_captures` and `canvas_operation` serialization to `AppState`**. Allow only one capture worker, serialize its browser navigation with all existing Canvas navigation commands, and avoid holding the Canvas session mutex for the entire run. On application restart, treat a stale `running` item as pending before resumption.
- [ ] **Step 7: Add Tauri commands** for start, resume, cancel, status, paginated items, retry of a retryable failed item, and getting/saving capture limits. Starting validates authentication and persists valid limits; unsupported ignored items cannot be retried as if they were accessible.
- [ ] **Step 8: Emit `canvas-capture-updated` after each persisted state change** with a sanitized `CaptureRun`; no raw URL query, fragment, credential, or cookie enters the event. Do not create lesson-study sessions during import.
- [ ] **Step 9: Run the focused Rust tests** with `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib canvas_capture::tests`; verify event payload serialization contains no query, fragment, credentials, cookies, or raw server error.

### Task 5: Add batch controls and per-course progress in the React app

**Files:**
- Create: `app/src/ui/CanvasCapturePanel.tsx`
- Create: `app/src/ui/canvasCapture.ts`
- Create: `app/scripts/canvas-capture.test.mjs`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/SubjectDetail.tsx`
- Modify: `app/src/ui/studyTypes.ts`
- Modify: `app/src/ui/styles.css`

**Interfaces:**
- `canvasCapture.ts` mirrors `CaptureRun`, `CaptureItem`, `CaptureCounts`, `CaptureRunStatus`, and `CaptureItemStatus` from Rust.
- `captureProgressLabel(run: CaptureRun): string` maps states/counts to user-facing Portuguese copy.
- `groupCaptureItemsByCourse(items: CaptureItem[]): CourseCaptureGroup[]` creates stable per-course result groups.
- `CanvasCapturePanel` receives latest run, paginated items, busy state, and `onStart`, `onResume`, `onCancel`, and `onRetry` callbacks.
- `CanvasCapturePanel` also receives/saves `CanvasCaptureLimits`; advanced controls change the documented run limits while concurrency stays fixed at one.

- [ ] **Step 1: Add Node tests** named `capture_progress_label_distinguishes_partial_auth_and_completed`, `capture_item_groups_preserve_course_module_order`, and `capture_status_copy_never_claims_official_attendance`.
- [ ] **Step 2: Run `npm test --prefix app` and confirm the new helper tests fail** before implementing progress mapping/grouping.
- [ ] **Step 3: Implement the typed capture DTOs and helper functions** in `canvasCapture.ts`; keep enum strings aligned with the Rust serialization contract.
- [ ] **Step 4: Build `CanvasCapturePanel`** with a native progress element, start/resume/cancel actions, per-course status and item-level retry/review state; explain that opening content may create ordinary Canvas access records and does not record attendance.
- [ ] **Step 5: Add advanced limit controls** for course pages, courses, items, PDF size, redirects, and per-item timeout; validate using backend bounds and keep concurrency fixed at one.
- [ ] **Step 6: Wire commands and the `canvas-capture-updated` event** in `App.tsx`. On completion, refresh imported materials and the local catalog without navigating away from the current screen.
- [ ] **Step 7: Update material rows and `materialsForEnrollment`** to use `material_associations`, show page/PDF source kind and capture date, and display shared content under each associated course without duplicating the Markdown file. Opening a captured item starts the normal local study session; bulk import itself never starts one.
- [ ] **Step 8: Run `npm test --prefix app` and `npm run build --prefix app`**; expect all helper tests to pass and TypeScript/Vite production build to succeed.

### Task 6: Document the operating flow and verify the integrated feature

**Files:**
- Modify: `app/README.md`
- Modify: `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`
- Modify: each affected semester index at `materias/fontes-uva/<periodo>/README.md`
- Verify: existing and newly added Rust/frontend tests

**Interfaces:**
- Documentation states the exact capture boundary, manual login requirement, statuses, pause/resume flow, unsupported formats, and distinction from official attendance.
- Implementation status records only observed coverage, with capture date and counts; no inaccessible course/item is described as downloaded.

- [ ] **Step 1: Update the README** with the authenticated Canvas setup, start/resume/cancel actions, local output paths, and per-item error workflow.
- [ ] **Step 2: Update the implementation status and each affected semester index** with source/date/status fields and explicit unverified/unsupported coverage, without touching or moving original academic files.
- [ ] **Step 3: Run focused complete checks:** `cargo fmt --manifest-path app/src-tauri/Cargo.toml -- --check`; `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib`; `cargo build --offline --manifest-path app/src-tauri/Cargo.toml --bin estuda-uva`; `npm test --prefix app`; `npm run build --prefix app`; `git diff --check`.
- [ ] **Step 4: Review the built UI against fixture-backed status/item data** and confirm progress, cancel/resume states, retry visibility, and material refresh behavior. Keep fixtures out of production behavior; report UI/build evidence separately from authenticated Canvas runtime evidence.
- [ ] **Step 5: If the student chooses to validate an authenticated course, capture one sample course after manual login** in the visible Chromium window; check its page/PDF text and verify that grades, attendance, submissions, and completion states were not changed. Do not start an all-course live sweep during verification.
- [ ] **Step 6: Recheck `git status --short` and per-file diffs**; keep pre-existing and newly captured local files untouched unless separately requested. Do not stage or commit this shared checkout.

## Coverage Map

| Specification area | Plan task |
|---|---|
| Course pagination, modules, lesson pages, safe links | Task 1 |
| SQLite run/item state, resume, safe source metadata | Task 2 |
| Canvas-page/PDF Markdown, hashes, conflicts, no raw PDF | Task 3 |
| Sequential worker, auth pause, cancellation, dedupe, progress | Task 4 |
| Start/resume/cancel UI, statuses, no attendance claims | Task 5 |
| Step-by-step documentation, build/test/runtime evidence | Task 6 |
| Official attendance and other remote actions excluded | Tasks 4–6 |
