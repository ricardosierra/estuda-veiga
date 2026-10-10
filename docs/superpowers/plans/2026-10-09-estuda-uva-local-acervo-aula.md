# Estuda UVA Local Archive and Aula Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make every existing Markdown file and supported local study document discoverable, searchable, and readable in Estuda UVA's Aula view without Canvas or duplicate downloads.

**Architecture:** A Rust `CatalogService` scans the configured Faculdade root with strict path and size limits, extracts supported text, and returns structured entries and selected document content through Tauri commands. The React Acervo presents the existing folder tree and opens Markdown in the Aula reader. This first plan provides the local read surface; the next plan attaches persistent three-way synchronization and the repository-local database.

**Tech Stack:** Rust 1.90, Tauri 2.12.1, `pdf-extract` 0.12.1, `chromiumoxide` 0.9.1 for existing import flows, React 19.3, TypeScript 7, `react-markdown` 10.1.0, `remark-gfm` 4.0.1. Probe `docx-lite` 0.2.0 and `iwork` 0.4.0 against the real originals before pinning either crate.

**Spec:** `docs/superpowers/specs/2026-10-09-estuda-uva-local-catalog-design.md`

## Global Constraints

- The configured repository root is the local archive; source paths are relative to its canonical path.
- Index every `.md` under the root, including project documentation, while classifying course lessons, academic records, planning, and app documentation separately.
- Index eligible academic JSON; exclude package/dependency manifests, Tauri/TypeScript configuration, build outputs, runtime profiles, and the synchronization mirror directory from the academic corpus.
- Exclude `.git`, `node_modules`, `target`, `dist`, and `.estuda-uva/mirrors` before traversal. Reject traversal and symlink escapes.
- Bound traversal depth to 16, source entries to 2,000, each source file to 64 MiB, and extracted index text to 128 MiB. Show every affected/unsupported item with a reason; never silently omit it.
- Read/render Markdown with raw HTML disabled; never execute content or remote resources.
- Local scan/search/preview performs no network requests and remains usable while Canvas is disconnected.
- Open any Markdown item in Aula view; preserve source path, headings, tables, relative links, and local images. Do not invent lesson content or academic facts.
- Existing PDFs and other binaries already in the archive remain at their current source paths. Newly downloaded Canvas PDFs are temporary extraction inputs; only Markdown is kept. No existing file is overwritten.
- Keep the current branch and all unrelated working changes. After validation, leave this scope unstaged and uncommitted for the user to review.

## Review Focus

- Symlink or `..` escape — test that all file reads stay under the canonical root.
- Malformed/oversized documents — test that one file produces a local error without hiding healthy entries.
- Markdown HTML and links — test that raw HTML is not executed and relative links cannot escape the library.
- Large/unsupported JSON and PDFs — test memory/count limits and preserve page boundaries/no-text state.
- Real DOCX and Pages originals — test extraction order/tables and byte-for-byte source preservation before choosing parser dependencies.

---

## File Structure

- `app/src-tauri/src/catalog/mod.rs`: public types and `CatalogService` facade.
- `app/src-tauri/src/catalog/files.rs`: bounded walk, path validation, and tree entries.
- `app/src-tauri/src/catalog/documents.rs`: text extraction for Markdown, JSON, HTML, TXT, ALG, PDF, DOCX, and Pages.
- `app/src-tauri/src/lib.rs`: app state and root-bounded Tauri commands.
- `app/src/ui/catalogTypes.ts`: shared IPC types.
- `app/src/ui/AcademicCatalog.tsx`: tree, search, filters, reader, and errors.
- `app/src/ui/App.tsx` and `app/src/ui/styles.css`: navigation and Aula integration.

### Task 1: Add bounded, safe document readers

**Files:**
- Create: `app/src-tauri/src/catalog/mod.rs`
- Create: `app/src-tauri/src/catalog/documents.rs`
- Modify: `app/src-tauri/Cargo.toml`
- Modify: `app/src-tauri/Cargo.lock`
- Modify: `app/src-tauri/src/lib.rs` (module declaration)
- Test: inline tests in `app/src-tauri/src/catalog/documents.rs`

**Interfaces:**
- `extract_document(path: &Path, bytes: &[u8]) -> Result<ExtractedDocument, DocumentError>`.
- `ExtractedDocument { text: String, format: DocumentFormat, page_boundaries: Vec<PageBoundary>, warnings: Vec<String> }`.
- Markdown/TXT/ALG/JSON use UTF-8 decoding with explicit replacement/error policy; JSON display is escaped text and academic JSON is parsed without dropping unknown fields.
- PDF uses existing `pdf::split_pdf_pages`; image-only PDFs return a no-selectable-text status.
- DOCX/Pages dependencies are pinned only after a read-only probe of `originais/paradigmas-de-programacao/trabalho2-paradigmas-ricardo.docx` and `originais/paradigmas-de-programacao/trabalho-paradigmas-ricardo.pages` succeeds.

- [ ] **Step 1: Add failing tests** `extracts_markdown_without_html_execution`, `extracts_json_without_reordering_payload`, `extracts_pdf_with_page_boundaries`, `reports_image_only_pdf`, and `extracts_existing_docx_and_pages_without_writing_source`.
- [ ] **Step 2: Run the focused test filter and confirm the parser cases fail before implementation.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib catalog::documents::tests`

- [ ] **Step 3: Probe the current DOCX and Pages files** using the candidate Rust crates; assert readable ordered content and unchanged SHA-256 source bytes. Pin only candidates that pass.
- [ ] **Step 4: Implement `extract_document`** with bounded byte input, ordered text/tables, PDF page boundaries, and per-format warnings.
- [ ] **Step 5: Run the focused document tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib catalog::documents::tests`

  Expected: all present formats return readable text or a specific unsupported/no-text reason; source files remain unchanged.
- [ ] **Step 6: Leave the reader module and selected parser dependencies in the working tree for review.**

### Task 2: Index the complete local Markdown corpus and supported materials

**Files:**
- Create: `app/src-tauri/src/catalog/files.rs`
- Modify: `app/src-tauri/src/catalog/mod.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Test: inline tests in `app/src-tauri/src/catalog/files.rs`

**Interfaces:**
- `CatalogService::new(root: PathBuf) -> Result<CatalogService, CatalogError>`.
- `CatalogService::refresh(&mut self) -> CatalogSnapshot`.
- `CatalogService::search(&self, query: &str, filter: CatalogFilter) -> Vec<CatalogSearchResult>`.
- `CatalogService::read_document(&self, relative_path: &str) -> Result<RenderedDocument, CatalogError>`.
- `CatalogService::read_asset(&self, relative_path: &str) -> Result<Vec<u8>, CatalogError>`.
- `CatalogSnapshot { root: String, scanned_at: String, status: CatalogStatus, entries: Vec<CatalogEntry>, errors: Vec<CatalogItemError> }`.
- `CatalogEntry { relative_path: String, parent_path: String, name: String, kind: EntryKind, course: Option<String>, semester: Option<String>, category: String, size_bytes: u64, searchable: bool }`.
- Only relative paths cross IPC. Catalog operations re-canonicalize every requested path before reading it.

- [ ] **Step 1: Add failing tests** `indexes_every_markdown_including_docs`, `excludes_configuration_json_and_generated_directories`, `preserves_relative_directory_tree`, `rejects_symlink_escape_and_parent_traversal`, and `reports_per_file_limit_and_parse_errors`.
- [ ] **Step 2: Run the focused file-catalog tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib catalog::files::tests`

  Expected: tests fail because `CatalogService` and bounded traversal are not implemented.
- [ ] **Step 3: Implement deterministic traversal and classification** using the exact limits in Global Constraints; skip symlinks and ignored roots before reading file contents.
- [ ] **Step 4: Add format extraction and local title/content search** over the current in-memory snapshot. Classify records from explicit path/metadata only; never infer enrollment from a file name.
- [ ] **Step 5: Run scanner, path-safety, and search tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib catalog::`

  Expected: every in-scope Markdown file is visible/searchable, supported materials open, and each excluded/failed item has a clear reason.
- [ ] **Step 6: Leave the bounded local catalog service in the working tree for review.**

### Task 3: Expose the catalog to Tauri

**Files:**
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src-tauri/src/storage.rs` (reuse configured library root only)
- Modify: `app/src-tauri/src/catalog/mod.rs`
- Test: catalog command tests in `app/src-tauri/src/lib.rs`

**Interfaces:**
- Add `catalog: Mutex<CatalogService>` to `AppState`, initialized with `AcademicStore::library_path()`.
- Register commands `local_catalog_snapshot`, `refresh_local_catalog`, `search_local_catalog(query, filter)`, `read_local_document(relative_path)`, and `read_local_asset(relative_path)`.
- Change `set_library_path(path)` to validate/canonicalize the existing directory, replace the catalog root, and return the first snapshot.
- The existing three-second `refresh()` poll remains free of catalog scans.

- [ ] **Step 1: Add command tests** `local_catalog_loads_with_canvas_disconnected`, `read_document_rejects_outside_root`, and `set_library_path_refreshes_only_after_validation`.
- [ ] **Step 2: Implement the root-bound Tauri commands** with per-item errors converted to safe display messages.
- [ ] **Step 3: Run all catalog command tests and existing Rust library tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib`

  Expected: all existing behavior remains passing and local catalog commands work without a Canvas session.
- [ ] **Step 4: Leave the IPC and app-state wiring in the working tree for review.**

### Task 4: Build the Acervo tree and Aula reader UI

**Files:**
- Create: `app/src/ui/catalogTypes.ts`
- Create: `app/src/ui/AcademicCatalog.tsx`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/styles.css`
- Test: existing frontend build and the app's local manual flow

**Interfaces:**
- `AcademicCatalog` receives a `CatalogSnapshot` and refresh/search/read callbacks.
- `RenderedDocument` returns entry metadata, Markdown/text content, page boundaries, and warnings.
- Acervo includes folder navigation, search, course/semester/type filters, unsupported/read errors, and path breadcrumbs.
- Aula renders Markdown via existing `react-markdown` and `remark-gfm`; raw HTML and remote images remain disabled. Relative links navigate only to catalog entries under the current root.

- [ ] **Step 1: Add the Acervo component** with typed IPC payloads, accessible names, loading/empty/partial/error states, folder tree, and search results.
- [ ] **Step 2: Wire startup and explicit refresh** to `local_catalog_snapshot` and `refresh_local_catalog`; do not put local scanning inside the Canvas status timer.
- [ ] **Step 3: Add the Aula reader** with heading outline, tables, page sections, local image references, source path, and course/semester labels where known.
- [ ] **Step 4: Run the production frontend build.**

  Run: `npm run build --prefix app`

  Expected: TypeScript and Vite finish with exit code 0.
- [ ] **Step 5: Open the packaged app with Canvas disconnected** and verify Markdown from a current course, prior-semester material, root academic record, and app documentation each appear in their distinct categories and open inside Aula.
- [ ] **Step 6: Leave the Acervo/Aula implementation in the working tree for review.**

## Handoff

After this plan passes, continue with `2026-10-09-estuda-uva-sync-git.md`; its scanner and IPC interfaces are the input to persistent document synchronization.
