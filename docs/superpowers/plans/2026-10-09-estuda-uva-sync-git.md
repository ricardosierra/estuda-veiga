# Estuda UVA Three-Way Sync and Git Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Keep app-managed academic records and eligible Markdown/JSON documents synchronized across the SQLite database in the selected Faculdade repository, Markdown, and JSON, with safe migration, conflict recovery, and Git visibility.

**Architecture:** Move the academic runtime store to `<workspace>/.estuda-uva.sqlite3`, preserving the existing Application Support database as a recoverable migration source and keeping machine-specific browser/settings data outside Git. A Rust `SyncEngine` tracks stable IDs, base hashes/revisions, tombstones, an outbox, and conflict snapshots; it reconciles exact source paths with generated counterpart projections. Filesystem watcher notifications trigger a debounced reconcile, while startup and manual full reconciliation remain the recovery authority.

**Tech Stack:** Rust 1.90, Tauri 2.12.1, `rusqlite` 0.37 with `bundled` and `backup`, `serde_json` 1, `sha2` 0.10, `notify` 8.2.0, React 19.3, TypeScript 7, Git CLI for read-only status.

**Spec:** `docs/superpowers/specs/2026-10-09-estuda-uva-local-catalog-design.md` and `docs/superpowers/specs/2026-10-09-catalogo-academico-sincronizacao-design.md`

## Global Constraints

- The authoritative workspace is the selected existing Git repository; keep the current `main` branch and do not create branches/worktrees/remotes.
- Store academic SQLite as `<workspace>/.estuda-uva.sqlite3`; ignore the database, `-wal`, `-shm`, lock/temp/journal files, secrets, profiles, and build output.
- Leave the old Application Support database byte-for-byte intact after migration and keep Chromium profile/settings machine-local.
- Every managed academic document/record has a stable ID, schema/revision, last-common hashes, source/projection paths, and provenance.
- App edits, external Markdown edits, and eligible JSON edits reconcile to all three stores; invalid inputs do not replace the last valid revision.
- Never use last-writer-wins for divergent edits; preserve both/all versions in a visible conflict.
- Deletion in one surface creates a tombstone; no record or file is removed across surfaces without explicit resolution.
- Use deterministic JSON serialization and atomic file replacement plus a recoverable database outbox.
- `.estuda-uva/mirrors/` holds generated, Git-visible document counterparts and is excluded from the source scan to prevent loops.
- App/dependency configuration JSON is not academic data and must never be rewritten by sync.
- The app reports Git status but does not stage, commit, push, initialize, or change branches automatically. Leave changes unstaged and uncommitted for the user to review.
- Create root `AGENTS.md` and `CLAUDE.md` independently only when each is missing; never replace an existing file or symlink.
- Preserve unrelated changes. Do not stage, commit, push, or publish; the user will review and decide whether to commit.

## Review Focus

- Simultaneous edits to the same record — test snapshots preserve all sides until explicit conflict resolution.
- App termination between database commit and file projection — test outbox replay and repeatability.
- Watcher echo storms, dropped native notifications, or new directories — test debounce/filtering and full startup/manual reconciliation.
- Existing database and new workspace database both contain rows — test that neither copy is silently discarded.
- Existing guidance files/symlinks and Git-ignored personal state — test byte-for-byte preservation and exact Git ignore/status behavior.

---

## File Structure

- `app/src-tauri/src/storage.rs`: workspace database open, versioned migrations, and existing app store queries.
- `app/src-tauri/src/sync.rs`: schemas, stable IDs, serialization, reconciliation, outbox, conflicts, tombstones, and snapshots.
- `app/src-tauri/src/sync_watcher.rs`: in-scope directory watch set, debounce, and reconcile triggers.
- `app/src-tauri/src/guidance.rs`: safe creation of missing root `AGENTS.md` and `CLAUDE.md`.
- `app/src-tauri/src/repository.rs`: read-only `git status` parsing and safe workspace metadata.
- `app/src-tauri/src/lib.rs`: state, startup migration, commands, and Canvas duplicate preflight.
- `app/src-tauri/Cargo.toml` and `app/src-tauri/Cargo.lock`: pinned `notify` and SQLite backup features.
- `.gitignore`: ignore only private/runtime artifacts; keep text mirrors visible to Git.
- `app/src/ui/SyncCenter.tsx`, `app/src/ui/syncTypes.ts`, `app/src/ui/App.tsx`, `app/src/ui/styles.css`: sync status, conflicts, and repository status.

### Task 1: Move the academic store into the selected workspace safely

**Files:**
- Modify: `app/src-tauri/Cargo.toml`
- Modify: `app/src-tauri/Cargo.lock`
- Modify: `app/src-tauri/src/storage.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Test: inline storage/migration tests in `app/src-tauri/src/storage.rs`

**Interfaces:**
- `AppConfig::open(app_data_dir: &Path) -> Result<AppConfig, StorageError>` owns machine-local library path and Chromium executable/profile settings.
- `AcademicStore::open_workspace(root: &Path, legacy_database: &Path) -> Result<AcademicStore, StorageError>` opens `<root>/.estuda-uva.sqlite3`, runs ordered `PRAGMA user_version` migrations, and imports legacy app tables with `rusqlite::backup::Backup` only when safe.
- `AcademicStore::database_path(&self) -> PathBuf` returns the canonical workspace database path.
- If both legacy and workspace databases contain independent records, copy into a temporary verified DB and merge by existing primary keys; never replace the workspace file. Leave the legacy source untouched.

- [ ] **Step 1: Add failing migration tests** `workspace_database_is_inside_root_and_ignored`, `backup_preserves_existing_app_tables_and_settings`, `migration_is_idempotent_and_legacy_file_unchanged`, and `nonempty_workspace_and_legacy_databases_are_merged_without_loss`.
- [ ] **Step 2: Run the focused storage tests and confirm migration behavior is not implemented.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib storage::tests`

- [ ] **Step 3: Add rusqlite's `backup` feature and implement online backup/migrations** using `PRAGMA user_version`; copy machine-specific settings to `AppConfig`, preserve the legacy file and its WAL/SHM state until a verified database opens.
- [ ] **Step 4: Update `run()` setup** to load configured workspace, migrate/open its database, and keep the Chromium profile under Tauri `app_data_dir`.
- [ ] **Step 5: Run migration tests and existing storage tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib storage::tests`

  Expected: source and destination integrity checks pass, record counts match/merge explicitly, and the source database hash is unchanged.
- [ ] **Step 6: Commit the workspace database migration after its focused tests pass.**

### Task 2: Implement stable sync records and SQLite–Markdown–JSON reconciliation

**Files:**
- Create: `app/src-tauri/src/sync.rs`
- Modify: `app/src-tauri/src/storage.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Test: inline tests in `app/src-tauri/src/sync.rs`

**Interfaces:**
- `SyncEngine::open(root: PathBuf, store: Arc<AcademicStore>) -> Result<SyncEngine, SyncError>`.
- `SyncEngine::reconcile(&self, reason: SyncReason) -> Result<SyncReport, SyncError>` performs bounded full reconciliation.
- `SyncEngine::update_record(&self, id: &str, source: RecordSurface, content: String) -> Result<SyncReport, SyncError>` validates an edit and projects a new revision.
- `SyncDocument { id: String, relative_path: String, media_type: String, content: String, metadata: serde_json::Value, revision: u64, version_vector: VersionVector, source: SourceProvenance }`; `VersionVector` records causality across paired devices.
- `SyncConflict { id: String, base_revision: u64, sqlite_value: Option<String>, markdown_value: Option<String>, json_value: Option<String>, changed_surfaces: Vec<RecordSurface> }`.
- For each source `.md`, write its JSON envelope to `.estuda-uva/mirrors/<relative-path>.json`; for eligible source `.json`, write its Markdown counterpart to `.estuda-uva/mirrors/<relative-path>.md`. Structured catalog JSON lives under `materias/catalogo/`; use reversible adapters and preserve unknown JSON fields.
- Persist `sync_documents`, `sync_surface_state`, `sync_outbox`, `sync_conflicts`, and `sync_tombstones` with stable IDs and per-surface hashes. Never ingest files under the mirror directory as sources.

- [ ] **Step 1: Add failing tests** `markdown_sqlite_json_round_trip`, `json_sqlite_markdown_round_trip_with_unknown_fields`, `sqlite_edit_updates_both_files`, `one_sided_external_edit_reconciles`, `same_field_edits_create_conflict`, and `unchanged_reconcile_is_idempotent`.
- [ ] **Step 2: Run the focused sync tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib sync::tests`

  Expected: tests fail because the sync schema and engine are not present.
- [ ] **Step 3: Add versioned sync tables and deterministic JSON envelopes** with `schemaVersion`, stable ID, relative path, revision, content hash, source metadata, and lossless body/unknown fields.
- [ ] **Step 4: Implement one-surface-change reconciliation** and atomic mirror writes. Commit database revision and outbox row together, write temp+fsync+rename, then mark the outbox complete.
- [ ] **Step 5: Implement three-way conflict and tombstone handling.** Preserve snapshots for all changed surfaces; deletion becomes a tombstone, not a delete. Resolution writes the chosen/merged content to every side as a new revision.
- [ ] **Step 6: Run sync tests including malformed JSON, bad frontmatter, conflict preservation, and restart replay.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib sync::tests`

  Expected: all three representations converge after one-sided edits; concurrent edits and invalid files retain the last valid bytes and show a conflict/error.
- [ ] **Step 7: Commit the core three-way sync engine after its focused tests pass.**

### Task 3: Reconcile external edits automatically without watcher loops

**Files:**
- Create: `app/src-tauri/src/sync_watcher.rs`
- Modify: `app/src-tauri/Cargo.toml`
- Modify: `app/src-tauri/Cargo.lock`
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src-tauri/src/sync.rs`
- Test: inline watcher tests in `app/src-tauri/src/sync_watcher.rs`

**Interfaces:**
- `SyncWatcher::start(root: PathBuf, trigger: impl Fn() + Send + Sync + 'static) -> Result<SyncWatcher, WatchError>`.
- `WatchSet::rebuild(root: &Path) -> Result<WatchSet, WatchError>` watches only in-scope directories/files and prunes `.git`, build/dependency/runtime roots, and `.estuda-uva/mirrors`.
- Coalesce filesystem events for 500 ms, then call `SyncEngine::reconcile(SyncReason::FilesystemEvent)`; self-origin writes are recognized by hashes and cannot echo indefinitely.
- Watch the workspace SQLite main/WAL/SHM files as change hints too; compare per-record serialized hashes and SQLite `data_version`, not whole-database file hashes, so app-internal/irrelevant writes do not become academic changes.
- Startup and `sync_now` always perform full reconciliation even if the watcher reported no events or its backend reports an error.

- [ ] **Step 1: Add failing tests** `watcher_debounces_burst`, `watcher_ignores_generated_mirror_and_git`, `external_sqlite_write_schedules_record_reconcile`, `app_sync_write_does_not_echo`, `new_in_scope_directory_is_added_to_watch_set`, and `missed_event_is_recovered_by_full_reconcile`.
- [ ] **Step 2: Add `notify = "=8.2.0"` and create a `RecommendedWatcher`** with callback-to-channel handoff; retain the watcher for the app lifetime.
- [ ] **Step 3: Build a pruned watch set** from the catalog's canonical in-scope paths; filter self-writes by comparing hashes in sync state.
- [ ] **Step 4: Add Tauri `sync_now()` and sync-status event emission** so watcher/manual/startup reconciliation share one engine and report pending/conflict/error counts.
- [ ] **Step 5: Run watcher tests and `cargo check`.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib sync_watcher::tests`

  Run: `cargo check --manifest-path app/src-tauri/Cargo.toml`

  Expected: event bursts trigger one reconcile; dropped events are repaired at next full scan; app-generated mirror writes do not trigger repeat sync.
- [ ] **Step 6: Commit watcher lifecycle and sync status events after the Rust checks pass.**

### Task 4: Bootstrap project guidance and integrate Git status

**Files:**
- Create: `app/src-tauri/src/guidance.rs`
- Create: `app/src-tauri/src/repository.rs`
- Create: `app/src/ui/syncTypes.ts`
- Create: `app/src/ui/SyncCenter.tsx`
- Modify: `.gitignore`
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/styles.css`
- Test: inline Rust tests and the frontend production build

**Interfaces:**
- `ensure_project_guidance(root: &Path) -> Result<GuidanceReport, GuidanceError>` uses exclusive creation for each missing root file; it never edits or follows an existing file/symlink.
- `repository_status(root: &Path) -> Result<RepositoryStatus, RepositoryError>` runs only fixed, read-only `git -C <root> status --short --untracked-files=normal -z --ignored=matching` arguments and parses paths without staging.
- `RepositoryStatus` separates tracked changes, untracked files, ignored runtime database/profile paths, and repository errors.
- `SyncCenter` shows last full reconciliation, pending outbox, per-file errors, three-way conflicts, tombstones, and Git status; it provides resolve/restore actions but no stage/commit/push buttons.

- [ ] **Step 1: Add failing bootstrap tests** `creates_only_missing_agents_and_claude`, `existing_guidance_bytes_are_unchanged`, and `existing_symlink_is_not_followed_or_replaced`.
- [ ] **Step 2: Add Git fixture tests** `reports_tracked_and_untracked_without_staging` and `database_wal_shm_and_credentials_are_ignored` using a temporary test repository.
- [ ] **Step 3: Add `.gitignore` patterns** for root `.estuda-uva.sqlite3`, `-wal`, `-shm`, sync temp/journal files, app browser profile/session, credentials, and builds; do not ignore `.estuda-uva/mirrors` or `materias/catalogo`.
- [ ] **Step 4: Implement safe guidance creation** with concise Portuguese templates that explain sync paths, no-redownload behavior, privacy, and local verification; create only missing files.
- [ ] **Step 5: Implement fixed-argument, read-only Git status** and wire the Tauri command. Treat missing Git/repository as a status, not as a sync failure.
- [ ] **Step 6: Build the SyncCenter conflict/repository UI** and run `npm run build --prefix app`.
- [ ] **Step 7: Run guidance/Git Rust tests and confirm existing files are unchanged and generated mirrors/database have the expected Git status.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib guidance::tests`

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib repository::tests`

  Expected: only missing guidance files are created; local SQLite is ignored; mirrors and JSON catalog remain visible as ordinary Git changes; no stage/commit/push occurs.
- [ ] **Step 8: Leave the bootstrap, ignore rules, Git status command, and sync UI unstaged and uncommitted after verification.**

### Task 5: Skip already imported Canvas resources before downloading

**Files:**
- Modify: `app/src-tauri/src/storage.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src-tauri/src/archive.rs`
- Modify: `app/src/ui/App.tsx`
- Test: inline storage/import tests in `app/src-tauri/src/lib.rs` and `app/src-tauri/src/archive.rs`

**Interfaces:**
- Add `external_source_id` to imported-material metadata and a lookup `find_material_by_source(course_id: &str, source_id: &str) -> Result<Option<ImportedMaterial>, StorageError>`.
- Check the selected course/resource ID before asking the browser for a download URL. For older imports without that ID, resolve the already observed Canvas URL, strip signatures/query secrets, then match course + normalized source URL/path against local metadata before `download_pdf`.
- Return a typed result `ExistingLocal | ImportedMarkdown(ImportedBundle)` so the UI opens an existing local lesson when present and only transfers a confirmed missing resource.
- Download a selected PDF into a per-job temporary location, convert it to page-delimited Markdown under the existing `materias/<course-folder>/materiais/` convention, and delete the temporary PDF on success, error, cancellation, and startup recovery. Use create-new/atomic Markdown writes; identical retries are idempotent and different existing content is preserved.
- If a PDF has no selectable text, create an explicit Markdown status with source metadata and remove the temporary PDF; do not invent content or retain the binary.
- Extend the explicit Canvas action to enumerate all accessible current, past, and future courses, modules, pages, assignments, announcements, files, and available grade/activity views with pagination. Report inaccessible/unsupported sections and per-item errors. Background app polling does not contact Canvas.

- [ ] **Step 1: Add failing tests** `known_canvas_resource_is_opened_without_download`, `legacy_normalized_url_is_matched_before_download`, `same_title_different_resource_is_not_skipped`, `identical_retry_keeps_one_markdown`, `temporary_pdf_is_removed_after_every_outcome`, `no_text_pdf_has_markdown_status_only`, and `candidate_path_collision_preserves_existing_bytes`.
- [ ] **Step 2: Implement schema migration and source-ID lookup** with a unique `(course_id, external_source_id)` index while preserving old rows that have no source ID.
- [ ] **Step 3: Add an external-source-ID preflight before requesting a transfer** and a normalized-URL fallback before calling the authenticated download path; open an existing local Markdown record directly.
- [ ] **Step 4: Refactor `archive::import_pdf` to convert bounded temporary bytes into searchable Markdown** with idempotent hashes and collision-safe paths; save Canvas source ID and sanitized URL for future preflight.
- [ ] **Step 5: Preserve the current HTTPS/redirect/signature/60 MiB/cancellation checks and write files only after URL and PDF validation.**
- [ ] **Step 6: Run import/storage tests and frontend build.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib`

  Run: `npm run build --prefix app`

  Expected: known files incur no PDF network request; missing files still import additively; existing bytes remain unchanged.
- [ ] **Step 7: Commit Canvas deduplication and local-open wiring after verification.**

## Execution Order

Finish Tasks 1–4 before Plan `2026-10-09-estuda-uva-portal-academico.md`. Task 5 may follow Task 2 and the local catalog plan; it is independent of the Lyceum reader.

## External References

- `notify` 8.2.0 recommended watcher and backend behavior: https://docs.rs/notify/8.2.0/notify/.
- rusqlite online backup feature and source/destination connection rules: https://docs.rs/rusqlite/0.37.0/rusqlite/backup/.
