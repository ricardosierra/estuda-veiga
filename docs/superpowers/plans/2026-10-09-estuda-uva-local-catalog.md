# Estuda UVA Local Academic Catalog Implementation Plan

> **SUPERSEDED — DO NOT EXECUTE.** This untracked draft describes a read-only catalog and a separately selected database. The approved design now requires a repository-local SQLite database and bidirectional SQLite–Markdown–JSON synchronization. Its historical detail is preserved here; use the three active plans linked below instead: `2026-10-09-estuda-uva-local-acervo-aula.md`, `2026-10-09-estuda-uva-sync-git.md`, and `2026-10-09-estuda-uva-portal-academico.md`.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Estuda UVA read, classify, search, and open local academic files and a separately selected SQLite database, preserve the Faculdade folder hierarchy in the app, and save explicitly imported Canvas PDFs with searchable Markdown companions.

**Architecture:** Rust owns bounded filesystem scanning, document extraction, deterministic academic classification, and read-only SQLite inspection. A `CatalogService` exposes typed snapshots and paging commands to a React Acervo view that mirrors in-scope relative paths without copying them. Existing Canvas import and isolated Chromium behavior remain separate; explicit imports add source PDF and Markdown files under the configured library root.

**Tech Stack:** Rust 1.90, Tauri 2.12.1, rusqlite 0.37, React 19.3, TypeScript 7, `pdf-extract` 0.12.1, `docx-lite` 0.2.0 and `iwork` 0.4.0 subject to the real-file parser spike.

**Spec:** `docs/superpowers/specs/2026-10-09-estuda-uva-local-catalog-design.md`

## Global Constraints

- “Local scans and previews are read-only. They never copy, rename, move, edit, or delete existing folder files or the selected academic SQLite database. They make no network requests.”
- “Only an explicit student-started import may fetch approved course PDFs and add app-managed PDF/Markdown files under the configured root.”
- “The app never uploads local files, database schema, or academic records to Canvas.”
- “The app does not read or copy passwords or cookies from the student's personal Chrome profile.”
- “Text found in Markdown, PDFs, database records, or other source files is academic data to display and classify. It is never executed or treated as instructions for the app or its operator.”
- “No AI, telemetry, upload, or third-party indexing service is introduced.”
- Preserve the app database as settings, Canvas state, calendar, imported-material metadata, and study activity; never import external academic rows into it.
- Scan the configured library at startup and on explicit refresh; never scan during the existing three-second Canvas status poll.
- Keep the app usable when Canvas is disconnected and when no academic SQLite file is selected.
- Preserve the current authenticated-session download checks, 60 MiB PDF download bound, cancellation behavior, and secret-safe error handling.
- Preserve the current branch and all existing working-tree edits. Do not create a branch or worktree.

## Scope Boundary

An additional draft in the working tree, `docs/superpowers/specs/2026-10-09-catalogo-academico-sincronizacao-design.md`, proposes a normalized curriculum model and bidirectional JSON–SQLite reconciliation with migrations and conflict handling. Those are separate from the approved read-only academic SQLite catalog in the linked spec and are not included here. Do not overwrite or stage that draft as part of this plan. If the JSON–SQLite synchronization is meant to ship in the same delivery, revise and approve its design before implementation.

## Review Focus

- Traversal and symlink escape from the configured root — `catalog::files::tests::catalog_rejects_paths_outside_root`.
- Oversized, unsupported, unreadable, and over-limit entries must produce a visible partial-scan status — `catalog::files::tests::scan_reports_per_entry_failures_and_limits`.
- Real DOCX and Pages extraction must preserve text order and tables without writing to the source — `catalog::documents::tests::extracts_existing_docx_and_pages_read_only`.
- Arbitrary SQLite identifiers, missing primary keys, pagination, and credential-like columns — `academic_db::tests::pages_unknown_tables_safely_and_masks_sensitive_columns`.
- Canvas cancellation, no-text PDFs, filename collisions, and database loss must not overwrite existing files or hide local materials — `imported_material_is_idempotent_and_preserves_raw_pdf` and `no_text_pdf_is_preserved`.

---

## File Structure

- `catalog/mod.rs`: public catalog types, snapshot/index lifecycle, and service facade.
- `catalog/files.rs`: bounded root walk, path validation, and relative directory entries.
- `catalog/classification.rs`: course, semester, category, and source association rules.
- `catalog/documents.rs`: local format readers and normalized text/page boundaries.
- `academic_db.rs`: read-only schema discovery, mapping, sensitive-value redaction, and paginated rows.
- `archive.rs`: additive storage of imported source PDFs and their Markdown companions.
- `lib.rs` and `storage.rs`: Tauri state/commands and the local setting for the selected database path.
- `AcademicCatalog.tsx`, `catalogTypes.ts`, `App.tsx`, and `styles.css`: React navigation, preview, search, and table browser.
- `docs/GUIA-ACERVO-LOCAL.md` and both README files: user workflow, supported data, privacy, limits, and recovery.

### Task 1: Validate and add read-only DOCX and Pages extraction

**Files:**
- Modify: `app/src-tauri/Cargo.toml`
- Modify: `app/src-tauri/Cargo.lock`
- Create: `app/src-tauri/src/catalog/mod.rs`
- Create: `app/src-tauri/src/catalog/documents.rs`
- Modify: `app/src-tauri/src/lib.rs` (module declaration only)
- Test: `app/src-tauri/src/catalog/documents.rs` (inline unit tests)

**Interfaces:**
- Produces `extract_document(path: &Path, bytes: &[u8]) -> Result<ExtractedDocument, DocumentError>`.
- `ExtractedDocument` contains ordered `text: String`, `format: DocumentFormat`, and optional page/section boundaries; parsing never saves a transformed source.
- DOCX candidate: `docx-lite` 0.2.0 `parse_document`/`extract_text_from_bytes`.
- Pages candidate: `iwork` 0.4.0 `Document::open` and read-only `text_storages()` access.

- [ ] **Step 1: Add focused failing parser tests** for the existing files `originais/paradigmas-de-programacao/trabalho2-paradigmas-ricardo.docx` and `originais/paradigmas-de-programacao/trabalho-paradigmas-ricardo.pages`. Assert that extracted text is non-empty, paragraph/table or text-storage order is retained, and the source file bytes are unchanged.
- [ ] **Step 2: Run the focused parser tests and confirm the expected missing-parser failures.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib catalog::documents::tests`

- [ ] **Step 3: Pin only the parser crates that pass the real-file probe.** Use `docx-lite = "=0.2.0"` and `iwork = "=0.4.0"` if both read the current originals; otherwise choose a compatible local parser before continuing. Do not call any mutating iWork API.
- [ ] **Step 4: Implement `extract_document`** for DOCX and Pages and preserve paragraphs, readable table cells, and text order; report parser limitations instead of fabricating text.
- [ ] **Step 5: Run the focused parser tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib catalog::documents::tests`

  Expected: both real originals yield ordered text; fixture sources remain byte-identical.
- [ ] **Step 6: Commit the parser adapter and pinned dependency lockfile.**

### Task 2: Build the bounded local catalog and deterministic document classifier

**Files:**
- Create: `app/src-tauri/src/catalog/mod.rs`
- Create: `app/src-tauri/src/catalog/files.rs`
- Create: `app/src-tauri/src/catalog/classification.rs`
- Modify: `app/src-tauri/src/catalog/documents.rs`
- Modify: `app/src-tauri/src/lib.rs` (module declaration only)
- Test: inline tests in `app/src-tauri/src/catalog/files.rs` and `app/src-tauri/src/catalog/classification.rs`

**Interfaces:**
- `CatalogService::new(root: PathBuf) -> CatalogService`.
- `CatalogService::refresh(&self) -> Result<CatalogSnapshot, CatalogError>`.
- `CatalogService::search(&self, query: &str, filter: &CatalogFilter) -> Vec<CatalogSearchResult>`.
- `CatalogService::read_document(&self, relative_path: &str) -> Result<RenderedDocument, CatalogError>`.
- `CatalogService::read_asset(&self, relative_path: &str) -> Result<Vec<u8>, CatalogError>`.
- `CatalogSnapshot` carries `root`, `scannedAt`, `status`, counts, and relative entries. `CatalogEntry` carries `path`, `parentPath`, `name`, `kind`, `fileType`, `sizeBytes`, `modifiedAt`, course/semester/category labels, `readStatus`, `searchable`, and optional `importedMaterialId`.
- `CatalogFilter` has optional `course`, `semester`, and `fileType`; `CatalogSearchResult` has a `CatalogEntry`, matched excerpt, and optional page number; `RenderedDocument` has entry metadata, extracted text, page boundaries, and warnings.
- Use explicit mapping from known course/material registers and folder names. Unknown course or semester stays `geral/não classificado`; never infer official enrollment from a file.

- [ ] **Step 1: Add failing scanner/classifier tests** for root academic registers, `docs/` excluding `docs/superpowers/`, recursive `materias/` and `originais/`, ignored internals (`.git`, `.superpowers`, `app`, `node_modules`, `dist`, `target`, `tmp`), stable relative paths, unsupported-file visibility, changed/deleted files after refresh, course/semester labels, and escaped symlink rejection.
- [ ] **Step 2: Run the focused scanner tests and confirm they fail before implementation.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib catalog::`

- [ ] **Step 3: Implement the scanner with explicit limits:** depth 16, at most 2,000 entries, at most 64 MiB per source file, and at most 128 MiB of extracted index text. Reaching a limit produces a partial status and visible limit entry; it must not silently drop data. Do not follow directory symlinks. Canonicalize a selected file and reject any path outside the configured root.
- [ ] **Step 4: Implement deterministic folder/semester/course/category classification** and extension-aware extraction for Markdown, TXT, HTML, VisuAlg, JSON, PDF, DOCX, and Pages. Use existing `pdf::split_pdf_pages`; preserve page boundaries and mark PDFs with no selectable text as visual-only. Index text in memory only.
- [ ] **Step 5: Implement title/content search and case/diacritic-insensitive course, semester, and file-type filters** over the current in-memory snapshot.
- [ ] **Step 6: Run focused scanner, classification, and parser tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib catalog::`

  Expected: supported files are searchable; every in-scope unsupported, oversized, or unreadable item remains visible with a reason; no indexed content is persisted.
- [ ] **Step 7: Commit the local catalog service.**

### Task 3: Add a read-only academic SQLite adapter

**Files:**
- Create: `app/src-tauri/src/academic_db.rs`
- Modify: `app/src-tauri/src/lib.rs` (module declaration only)
- Test: inline tests in `app/src-tauri/src/academic_db.rs`

**Interfaces:**
- `AcademicDatabase::open_read_only(path: &Path) -> Result<AcademicDatabase, AcademicDatabaseError>`.
- `AcademicDatabase::schema(&self) -> Result<AcademicDatabaseSchema, AcademicDatabaseError>`.
- `AcademicDatabase::table_page(&self, table: &str, offset: u64, limit: u32) -> Result<AcademicTablePage, AcademicDatabaseError>`.
- `AcademicDatabaseSchema` contains database path, table names, column names/types, primary-key flags, bounded row-count summaries, and supported category labels.
- `AcademicTablePage` contains source table, offset, bounded limit, row-count summary, columns, and JSON-safe row values. Credential-like values are redacted in Rust before returning to the UI; BLOBs are represented by byte count, not raw bytes.
- Row-count summary is exact through 10,000 rows and “10,000+” above that bound; paging remains available independently of the summary.
- Known Portuguese/English table and column aliases classify course/program/enrollment status (including historical/evadido), semester, grade, attendance, schedule, and materials. Unmatched tables remain browsable as `não classificado`.

- [ ] **Step 1: Add failing SQLite fixture tests** for schema/table discovery, primary-key detection, recognized and unknown academic categories, row pagination (default 50, hard maximum 100), BLOB handling, masking `senha/password/token/secret/cookie` columns, a table identifier containing quotes, and an unchanged database-file hash after reads.
- [ ] **Step 2: Run the focused adapter tests and confirm they fail before implementation.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib academic_db::tests`

- [ ] **Step 3: Implement `open_read_only`** with `rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY`; never create, migrate, or write the selected academic database.
- [ ] **Step 4: Implement schema discovery** from `sqlite_master`, `PRAGMA table_info`, and foreign-key metadata. Avoid full-table `COUNT(*)`: count through a bounded subquery up to 10,001 rows and show `10.000+` for larger tables. Quote only identifiers discovered from the opened schema, bind pagination values, and cap `limit` at 100.
- [ ] **Step 5: Implement row serialization and classification.** Redact sensitive-name columns before producing a `AcademicTablePage`; keep unknown table data navigable without assigning guessed course/semester meaning.
- [ ] **Step 6: Run focused adapter tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib academic_db::tests`

  Expected: every table can be browsed by bounded pages; the database remains unchanged and sensitive values never reach returned rows or logs.
- [ ] **Step 7: Commit the read-only database adapter.**

### Task 4: Connect the catalog to Tauri state and settings

**Files:**
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src-tauri/src/storage.rs`
- Modify: `app/src-tauri/src/catalog/mod.rs`
- Modify: `app/src-tauri/src/academic_db.rs`
- Test: inline tests in `app/src-tauri/src/storage.rs` and the catalog/SQLite modules

**Interfaces:**
- Add `catalog: CatalogService` to `AppState`; initialize it from `AcademicStore::library_path()` during Tauri setup.
- Tauri commands: `local_catalog_snapshot() -> Result<CatalogSnapshot, String>`, `refresh_local_catalog() -> Result<CatalogSnapshot, String>`, `search_local_catalog(query, filter) -> Result<Vec<CatalogSearchResult>, String>`, `read_local_document(relative_path) -> Result<RenderedDocument, String>`, `read_local_asset(relative_path) -> Result<Vec<u8>, String>`, `set_academic_database_path(path) -> Result<AcademicDatabaseStatus, String>`, and `academic_database_table_page(table, offset, limit) -> Result<AcademicTablePage, String>`.
- Change `set_library_path(path)` to return `Result<CatalogSnapshot, String>` after updating the catalog root and performing the first scan for that root.
- `AcademicDatabaseStatus` includes `notSelected`, `scanning`, `ready`, `unreadable`, and `partial`, plus a safe message and table summary.
- Persist only the selected academic database path under `settings.academic_database_path`. Load it at startup; never persist its schema or rows.
- Updating `library_path` updates the catalog root. Startup and an explicit refresh rebuild the local snapshot; the three-second Canvas status poll never scans files.

- [ ] **Step 1: Add failing settings and command-level tests** for restoring a configured external SQLite path, keeping the app DB unchanged by catalog reads, rejecting an invalid database path, updating the catalog root, and isolating local scan errors from Canvas status.
- [ ] **Step 2: Run focused Rust tests and confirm the missing settings/commands.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib`

- [ ] **Step 3: Persist and restore only `academic_database_path`** through existing `AcademicStore::get_setting`/`set_setting`; validate the selected path is an existing regular SQLite file before saving.
- [ ] **Step 4: Add the catalog service to `AppState` and register the Tauri commands.** Each read command delegates to the root-bound service or read-only adapter; errors remain source-specific.
- [ ] **Step 5: Make `set_library_path` update the service root and return a refreshed local snapshot** without altering any existing folder contents.
- [ ] **Step 6: Run the focused Rust tests and `cargo check`.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib`

  Run: `cargo check --manifest-path app/src-tauri/Cargo.toml`

  Expected: app setup succeeds with no external database selected; folder reads work with Canvas disconnected; bad external database state does not hide local documents.
- [ ] **Step 7: Commit Tauri wiring and settings.**

### Task 5: Replace the Acervo empty state with the synchronized local catalog UI

**Files:**
- Create: `app/src/ui/catalogTypes.ts`
- Create: `app/src/ui/AcademicCatalog.tsx`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/styles.css`
- Test: `app` TypeScript/Vite production build

**Interfaces:**
- `AcademicCatalog` receives `snapshot`, optional `materials`, and async callbacks for refresh, search, document read, asset read, database selection, and SQLite table paging.
- Add typed frontend equivalents for `CatalogSnapshot`, `CatalogEntry`, `CatalogFilter`, `RenderedDocument`, `AcademicDatabaseStatus`, `AcademicDatabaseSchema`, and `AcademicTablePage` in `catalogTypes.ts`.
- Remove the corresponding duplicated inline types from `App.tsx` and import the shared catalog types.
- The root Acervo shows a folder tree with the same in-scope relative path structure; selecting a folder filters its children, and selecting a file loads only that file's content.
- The SQLite view selects the file via the native Tauri dialog and shows tables/columns/categories with 50-row pages; server-side redaction is treated as authoritative.

- [ ] **Step 1: Build the React catalog component and type adapters** for folder navigation, document preview, search, course/semester/type filters, unsupported-file reasons, scan status, and the separate SQLite table browser.
- [ ] **Step 2: Wire `App.tsx` to the catalog commands.** Load the first catalog snapshot on startup and after a root/database change; keep `refresh()`'s three-second Canvas status poll free of local scans; make the explicit refresh button call `refresh_local_catalog`.
- [ ] **Step 3: Add the native `.sqlite`, `.sqlite3`, and `.db` file picker** and persist the selected path via `set_academic_database_path`; no selected DB leaves local files fully usable.
- [ ] **Step 4: Render Markdown with raw HTML disabled, plain/source/JSON as escaped text, HTML as extracted text, and images only from `read_local_asset`.** Intercept relative links for safe in-app navigation and label external official-source links separately.
- [ ] **Step 5: Connect a scanned imported Markdown path to its existing `ImportedMaterial` guided-lesson action** so a document appears once in the tree while retaining the existing open-aula behavior.
- [ ] **Step 6: Run the frontend production build.**

  Run: `npm run build --prefix app`

  Expected: TypeScript and Vite complete without errors; Acervo can render with no Canvas session and no academic SQLite selected.
- [ ] **Step 7: Commit the Acervo UI and typed IPC contracts.**

### Task 6: Preserve source PDFs during explicit Canvas imports

**Files:**
- Modify: `app/src-tauri/src/archive.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/styles.css`
- Test: inline tests in `app/src-tauri/src/lib.rs` and `app/src-tauri/src/archive.rs`

**Interfaces:**
- Keep `import_material(job_id, course, module_title, title, material_id) -> Result<ImportedBundle, String>` and its current host, redirect, signature, size, isolated-profile, and cancellation checks.
- Expand `ImportedBundle` to `{ material: Option<ImportedMaterial>, lesson_markdown: Option<String>, original_pdf_path: String, status: "ready" | "visualOnly" }`; the UI opens the guided lesson only for `ready` and shows a local no-text status for `visualOnly`.
- Add `write_new_or_equal_bytes(path: &Path, contents: &[u8]) -> Result<PathBuf, ArchiveError>`; equal retry is idempotent, different existing bytes get a collision-safe suffix, and no existing file is overwritten.
- Resolve the course folder from explicit aliases in course/material registers and existing directory names; prefer a unique mapped folder, otherwise use a normalized course slug and append the Canvas course ID only on collision.
- Save the downloaded PDF under `originais/<course-folder>/<safe-title>.pdf`; save the page-delimited text under `materias/<course-folder>/materiais/<safe-title>.md` and keep existing lesson/SQLite behavior.

- [ ] **Step 1: Add failing import tests** asserting the original PDF bytes and Markdown companion exist in the matching course folders; an identical retry repairs a missing companion without duplicating files; different existing PDF/Markdown content is preserved; no-text PDFs keep the original and an explicit no-text Markdown status; invalid PDFs write no files; cancellation and download failure preserve existing library entries.
- [ ] **Step 2: Run focused archive/import tests and confirm they fail before implementation.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib imported_material_is_idempotent_and_preserves_raw_pdf`

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib no_text_pdf_is_preserved`

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib invalid_pdf_does_not_create_archive_output`

- [ ] **Step 3: Implement collision-safe byte writes for the PDF source** using the existing create-new/atomic pattern; do not use `fs::write` on a user path.
- [ ] **Step 4: Update `archive::import_pdf`** to preserve the original PDF and its Markdown companion under the matching folder tree even on an idempotent retry, link readable imports to the existing guided lesson, and represent no-selectable-text PDFs explicitly without fabricating content.
- [ ] **Step 5: Keep the current isolated app Chromium profile and download protections intact.** Do not add password/cookie copying, arbitrary-host downloading, or local-file uploads.
- [ ] **Step 6: Run focused archive/import tests and Rust checks.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib imported_material_is_idempotent_and_preserves_raw_pdf`

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib no_text_pdf_is_preserved`

  Run: `cargo check --manifest-path app/src-tauri/Cargo.toml`

  Expected: source PDFs and companions persist additively; cancellation/errors are item-scoped; local catalog use is independent from Canvas and app SQLite rows.
- [ ] **Step 7: Commit the additive PDF preservation change.**

### Task 7: Document the workflow and verify the packaged app

**Files:**
- Create: `docs/GUIA-ACERVO-LOCAL.md`
- Modify: `app/README.md`
- Modify: `README.md`

**Interfaces:**
- The guide documents source selection, folder-tree refresh, supported formats, course/semester identification rules, SQLite read-only schema/table browsing, masking, unsupported items, Canvas PDF import, output folders, limits, and errors.
- State clearly that academic records stay local and that documents are data, never operational instructions.

- [ ] **Step 1: Write the Portuguese user guide** with the exact app steps for selecting the Faculdade root and academic SQLite file, refreshing, searching, opening files, paging SQLite rows, and explicitly importing a Canvas PDF.
- [ ] **Step 2: Update both READMEs** with the catalog behavior, supported/unsupported formats, and preservation path for downloaded PDF plus Markdown.
- [ ] **Step 3: Run the full planned validation commands.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib`

  Run: `cargo check --manifest-path app/src-tauri/Cargo.toml`

  Run: `npm run build --prefix app`

  Expected: all Rust tests, Rust check, and frontend build pass.
- [ ] **Step 4: Build the Tauri app and manually verify the acceptance flow** with Canvas disconnected: the catalog loads nonzero academic files, both current course folders are searchable, a Markdown/TXT/PDF/DOCX/Pages file opens, folder refresh reflects a harmless added/removed fixture, a selected SQLite fixture shows paginated classified and unclassified rows, and app/academic databases remain separate.

  Run: `npm run tauri --prefix app -- build`

- [ ] **Step 5: Restore the configured library root to `/Users/sierra/Documents/Faculdade` after fixture-based verification; do not leave temporary paths or databases in app settings.**
- [ ] **Step 6: If an authenticated Canvas session is available, import one selected course PDF and verify both original PDF and Markdown paths, cancel/retry behavior, and that no local files are sent outbound.** If the session is not authenticated, record that portion as not exercised rather than claiming success.
- [ ] **Step 7: Open the packaged app for user review and commit the Portuguese guide and README updates.**

## Parser References

- [docx-lite 0.2.0](https://docs.rs/docx-lite/0.2.0/docx_lite/) exposes byte-based extraction and structured paragraphs/tables; use it only if the probe preserves the current DOCX source.
- [iwork 0.4.0](https://docs.rs/iwork/0.4.0/iwork/) exposes `Document::open` and read-only `text_storages()` for Pages; do not call its write/edit API.
