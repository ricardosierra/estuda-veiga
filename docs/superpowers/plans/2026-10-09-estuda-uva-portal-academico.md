# Estuda UVA Academic Model and Full Browser Refresh Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Capture all accessible read-only Canvas and UVA/Lyceum academic information—courses, modules, lessons, materials, grades, attendance, curriculum, progress, schedules, and upcoming requirements—with provenance, then synchronize it through the approved local SQLite–Markdown–JSON contract.

**Architecture:** Add a domain model that separates UVA programs/curriculum offerings, canonical shared subjects, Canvas courses, and the student's outcomes in each program. Extend the existing visible, app-isolated Chromium process using Rust/CDP through `chromiumoxide`; maintain a separate Lyceum page and independent status alongside Canvas. A user-started refresh traverses accessible Canvas courses and official portal sections with pagination, records per-section coverage, and never submits changes to institutional systems.

**Tech Stack:** Rust 1.90, Tauri 2.12.1, `chromiumoxide` 0.9.1/CDP, existing `scraper` 0.22, `rusqlite` 0.37, `serde`/`serde_json`, React 19.3, TypeScript 7. No Playwright, Selenium, WebDriver, Node sidecar, or Node browser automation.

**Spec:** `docs/superpowers/specs/2026-10-09-estuda-uva-local-catalog-design.md` and `docs/superpowers/specs/2026-10-09-catalogo-academico-sincronizacao-design.md`

## Global Constraints

- Use the existing Rust/CDP `chromiumoxide` session and its visible app-owned profile. Do not add Playwright, Selenium, WebDriver, or Node browser automation. Never read or copy cookies/passwords from a personal Chrome/Firefox profile.
- Open the official portal at `https://portalaluno.uva.br/Aluno/PortalAluno`; use only observed UVA/Lyceum navigation and allowlisted official hosts (`portalaluno.uva.br`, `sis.lyceum.com.br`, and observed official UVA subdomains).
- Canvas and portal navigation start only from explicit UI actions and cover every available course/section/page with pagination. Polling and local filesystem sync never browse UVA services.
- Newly fetched Canvas PDFs use a bounded per-job temporary path for text extraction and are removed after success, error, cancellation, and restart recovery. Persist the Markdown result and safe provenance only; do not persist unsupported source binaries.
- Portal operations are explicit and read-only: never submit enrollment, withdrawal, equivalency, profile, service, or financial changes.
- Keep Canvas status independent from Lyceum status and distinguish Canvas courses from UVA curricular offerings.
- Every captured grade, attendance, enrollment, workload, and completion fact includes program/term where available, source URL/label, and capture time.
- Preserve missing values, spelling as displayed, unknown fields, and portal-vs-course-hour discrepancies. Never invent course-level credit from aggregate hours or infer equivalency from similar names.
- Reuse an authenticated app-owned browser profile if its session is valid; otherwise leave the visible login/MFA page for manual completion. The user explicitly identified a credential note inside this Faculdade folder for login; consume it only in the visible session and never print, log, index, persist, or sync its contents.
- Persist academic facts via `SyncEngine`; browser cookies/session IDs/profile remain in app data and outside JSON/Git.
- After the complete requested scope passes verification, commit only its files on the current branch; do not push, publish, or include unrelated changes.

## Review Focus

- Lyceum unauthenticated/expired state — keep local archive and sync usable and present the isolated sign-in page.
- SPA redirects, pagination, and unexpected table structure — report a per-page parse error and preserve prior valid facts.
- Multiple program links and similar course names — keep program and offering IDs separate; never promote a candidate equivalence.
- Missing grades/attendance or summary-only workload — store explicit unknown/aggregate facts with their source, not fabricated values.
- Secret-bearing URL/cookie/error text — strip query/fragment/userinfo and prevent raw credentials from reaching logs, Markdown, or JSON.
- Canvas PDF conversion or cancellation — persist only Markdown and provenance; remove all temporary PDF bytes on every exit path.
- Missing session or partially inaccessible course/portal sections — keep the local catalog available and report exactly what was and was not captured.

---

## File Structure

- `app/src-tauri/src/academic.rs`: stable typed institution/program/curriculum/offering/subject/trajectory/source models.
- `app/src-tauri/src/browser.rs`: one visible Rust/CDP browser process/profile with independent Canvas and Lyceum pages/status.
- `app/src-tauri/src/lyceum.rs`: official portal navigation contract, captured-table normalization, status/errors, and source URL sanitization.
- `app/src-tauri/src/lyceum.rs`: Rust/CDP navigation, rendered-page capture, portal normalization, status/errors, and source URL sanitization.
- `app/src-tauri/src/storage.rs`: versioned academic tables/queries in the workspace database.
- `app/src-tauri/src/sync.rs`: project academic records into `materias/catalogo/*.json` and Markdown views.
- `app/src-tauri/src/lib.rs`: commands for visible login/status, full Canvas refresh, full portal refresh, cancellation, and provenance-aware SyncEngine writes.
- `app/src/ui/AcademicOverview.tsx`, `app/src/ui/AcademicPortal.tsx`, `app/src/ui/App.tsx`, `app/src/ui/styles.css`: academic summary, source comparisons, and portal action.
- `app/src-tauri/Cargo.toml` and `Cargo.lock`: retain and extend the existing Rust browser stack without adding a browser-automation sidecar.
- `materias/catalogo/`: versioned JSON data (`disciplinas.json`, `relacoes.json`, `curriculos/*.json`, `trajetorias.json`); existing local sources stay where they are, while newly downloaded PDFs are temporary only.

### Task 1: Add stable academic program, curriculum, subject, and trajectory entities

**Files:**
- Create: `app/src-tauri/src/academic.rs`
- Modify: `app/src-tauri/src/storage.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Test: inline model/storage tests in `app/src-tauri/src/academic.rs`

**Interfaces:**
- `Institution { id, name, official_domain }`.
- `AcademicProgram { id, institution_id, official_code, name, status, source }`.
- `CurriculumOffering { id, program_id, curriculum_version, official_code, display_name, workload_hours, term, source }`.
- `CanonicalSubject { id, display_name, aliases, source }`.
- `OfferingRelation { id, left_offering_id, right_offering_id, relation_kind, evidence, source, confidence }`, where `relation_kind` is `same_subject | partial_overlap | equivalence_candidate | equivalence_confirmed`.
- `StudentOutcome { id, program_id, offering_id, enrollment_status, term, grade, attendance, workload, completion, source, observed_at }`; absent values are `None`, never zero by default.
- `AcademicSource { kind, url, label, captured_at, source_record_id }` strips URL query/fragment/userinfo before persistence.
- Use stable IDs independent of display-name/path ordering; do not merge Canvas course IDs with UVA program/offering IDs.

- [ ] **Step 1: Add failing model tests** `same_title_offerings_keep_program_identity`, `subject_can_reference_multiple_offerings`, `unconfirmed_overlap_does_not_transfer_completion`, `missing_grade_is_unknown_not_zero`, and `source_url_drops_credentials_and_query`.
- [ ] **Step 2: Add versioned SQLite migrations** for institutions, programs, curriculum offerings, canonical subjects, relations, outcomes, assessments, attendance, workload, and source provenance without changing Canvas `courses` semantics.
- [ ] **Step 3: Implement models, repositories, and deterministic JSON projections** using the shared `SyncEngine` from the sync plan; write `materias/catalogo/disciplinas.json`, `relacoes.json`, curriculum JSON, `trajetorias.json`, and readable Markdown views.
- [ ] **Step 4: Import explicit facts from `REGISTRO_ACADEMICO.md` and `ENGENHARIA_DE_COMPUTACAO.md`** through a versioned parser/fixture. Preserve source wording, unknown grades, source dates, and both programs; retain the 330 h/416 h/3.118 h and 1.446 h/3.601 h source figures without inventing course-level completions.
- [ ] **Step 5: Run academic schema/model and sync projection tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib academic::tests`

  Expected: records round-trip through SQLite/JSON/Markdown, stable IDs persist, and every unknown/discrepancy remains explicit.
- [ ] **Step 6: Commit the academic entity model and local catalog seed after their focused verification passes.**

### Task 2: Extend the isolated Rust browser session for Canvas and Lyceum

**Files:**
- Modify: `app/src-tauri/src/browser.rs`
- Modify: `app/src-tauri/src/lib.rs`
- Test: browser session and status tests in `app/src-tauri/src/browser.rs`

**Interfaces:**
- `BrowserSession` owns one `Browser`, one CDP handler task, the current app-data profile, a Canvas page, and an optional Lyceum page.
- `open_lyceum() -> Result<PortalStatus, BrowserError>` creates/navigates a separate visible page in the existing app-owned process and checks the redirect host before continuing.
- `lyceum_page() -> Result<Page, BrowserError>` returns only the app-owned Lyceum tab. It never reads another application's cookies/profile.
- Keep Canvas and Lyceum statuses separate even though they share the existing isolated Chromium process and profile.

- [ ] **Step 1: Add failing session tests** `canvas_and_lyceum_status_are_independent`, `lyceum_page_reuses_app_owned_browser`, `portal_redirect_rejects_unapproved_host`, and `closing_session_closes_handler_and_pages`.
- [ ] **Step 2: Refactor `CanvasSession` into a shared `BrowserSession`** that retains the current executable/profile, browser handler task, and Canvas page while adding a separate Lyceum page.
- [ ] **Step 3: Add visible Lyceum open/status commands** and keep all portal interaction in Rust/CDP using the existing browser dependency.
- [ ] **Step 4: Confirm no Playwright, Node sidecar, Selenium, or WebDriver dependency is added.**
- [ ] **Step 5: Run focused browser/session tests and build the local app.**

  Expected: Canvas still works; Lyceum opens visibly in a second page under the same app-owned profile and has independent status, with no Playwright runtime.

### Task 3: Refresh every accessible Canvas course and persist only Markdown

**Files:**
- Modify: `app/src-tauri/src/browser.rs`
- Modify: `app/src-tauri/src/browser.rs`, `canvas.rs`, `archive.rs`, `download.rs`, `pdf.rs`, `storage.rs`, and `lib.rs`
- Modify: `app/src/ui/App.tsx` and `app/src/ui/styles.css`
- Test: Canvas traversal/import tests in `canvas.rs`, `archive.rs`, and the automation bridge

**Interfaces:**
- `BrowserSession::refresh_canvas_catalog() -> Result<BrowserCanvasCapture, BrowserError>` visits every Canvas course shown as current, past, or future, including paginated course lists. It captures modules, pages, assignments, announcements, files, and available grade/activity views without submitting forms.
- `BrowserCanvasCapture { courses: Vec<CanvasCourseCapture>, pages_read: u32, inaccessible_sections: Vec<SectionFailure>, captured_at: String }`; each course and section carries its external Canvas ID, sanitized source URL, readable text, materials, and explicit completeness state.
- `CanvasRefreshReport { courses_seen: u32, sections_read: u32, markdown_created: u32, reused_local: u32, unsupported: u32, errors: Vec<RefreshItemError>, complete: bool }` reports per-item outcomes; `complete` is true only when every available section was read.
- `refresh_canvas_catalog() -> Result<CanvasRefreshReport, String>` is a user-started Tauri command. It checks local source ID/hash/path first and transfers only missing supported material.
- For authenticated downloads, the existing visible app-owned Chromium page fetches only the selected resource and writes bounded bytes to a generated cache temp path. Rust converts PDFs with the existing `pdf::split_pdf_pages`, writes only a collision-safe Markdown lesson plus source metadata, and removes temporary bytes with a Rust cleanup guard on success, failure, cancellation, and next-launch recovery.
- New Canvas material paths follow existing course folders under `materias/<course-folder>/materiais/`; local same-source notes are reused and never overwritten. Unsupported binary attachments remain metadata/link entries only.

- [ ] **Step 1: Add failing tests** `refresh_lists_current_past_and_future_courses`, `traverses_course_sections_and_pagination`, `reports_an_inaccessible_section_without_claiming_complete`, `existing_source_id_skips_transfer`, `pdf_import_persists_markdown_only`, `temporary_pdf_is_removed_on_success_error_cancel_and_restart`, `no_text_pdf_gets_status_markdown`, and `existing_markdown_collision_is_preserved`.
- [ ] **Step 2: Implement Rust/CDP traversal** for course lists, modules/pages/assignments/announcements/files/grades and safe pagination; only follow allowlisted `uva.instructure.com` links observed in the visible app session.
- [ ] **Step 3: Add authenticated bounded resource download** through the existing Chromium session, enforce HTTPS/redirect/signature/60 MiB checks in Rust, and permit only generated temp paths under the app cache.
- [ ] **Step 4: Refactor `archive::import_pdf`** to parse temporary bytes, create page-delimited Markdown and provenance, and unconditionally remove PDF bytes; store source IDs and normalized safe URLs for future no-download checks.
- [ ] **Step 5: Add the explicit “Atualizar Canvas” action** with progress, cancellation, per-course/per-file outcomes, and separate login/session status. Do not call it from the three-second overview polling or local watcher.
- [ ] **Step 6: Run focused Canvas traversal/import checks and `npm run build --prefix app`.**

  Expected: accessible course material becomes searchable Markdown; duplicate resources make no download; no source PDF remains in the archive or app cache after a job finishes.

### Task 4: Navigate and capture every available UVA/Lyceum academic section

**Files:**
- Create: `app/src-tauri/src/lyceum.rs`
- Modify: `app/src-tauri/src/browser.rs` and `app/src-tauri/src/lib.rs`
- Test: portal parser and navigation tests in `app/src-tauri/src/lyceum.rs`

**Interfaces:**
- `PortalStatus { state, message, current_url, pages_read, inaccessible_sections }`, independent states `disconnected | waiting_for_login | authenticated | partial | error`.
- `BrowserSession::refresh_lyceum_catalog() -> Result<PortalCapture, BrowserError>` starts from `https://portalaluno.uva.br/Aluno/PortalAluno`, navigates visible official links only, and returns rendered pages/tables with sanitized URLs and pagination results.
- `PortalCapture { programs: Vec<ProgramCapture>, pages_read: u32, inaccessible_sections: Vec<SectionFailure>, warnings: Vec<PortalPageError>, captured_at: String }`; records include recognized typed fields and unknown visible columns/raw labels.
- `refresh_lyceum_catalog() -> Result<AcademicSyncReport, String>` starts only from a user action, maps each captured value to the academic model/SyncEngine, and never submits portal changes.
- Allow only `portalaluno.uva.br`, `sis.lyceum.com.br`, and official UVA hosts discovered through visible navigation. Strip query, fragment, and userinfo before storing URLs. Bound traversal to 50 pages per source operation and 10,000 rows per page.

- [ ] **Step 1: Add fixture-based failing tests** for Portal do Aluno account cards, Lyceum curriculum tables, grade/assessment tables, attendance/frequency, pagination, unknown columns, expired/login state, hostile redirects, and secret-query removal.
- [ ] **Step 2: Implement `refresh_lyceum_catalog` in Rust/CDP** with observed-link traversal, host allowlists, pagination loop detection, and explicit per-section errors.
- [ ] **Step 3: Parse visible HTML/table rows with existing `scraper` 0.22** and map known labels to typed academic fields; retain unknown labels and never fill missing grades/attendance as zero.
- [ ] **Step 4: Persist captured records through `SyncEngine`** with stable source IDs and capture provenance; failed/partial page reads do not erase prior valid facts.
- [ ] **Step 5: Run portal parser and provenance tests.**

  Expected: captures preserve page, row, source, term, and program context; unauthenticated or incomplete pages leave local history intact and visibly partial.

### Task 5: Connect captured facts to the synchronized course history and Aula interface

**Files:**
- Create: `app/src/ui/AcademicOverview.tsx`
- Create: `app/src/ui/AcademicPortal.tsx`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/styles.css`
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src-tauri/src/academic.rs`
- Test: academic mapping tests, frontend production build, and packaged-app flow

**Interfaces:**
- Commands: `academic_overview()`, `academic_programs()`, `canvas_refresh_status()`, `refresh_canvas_catalog()`, `lyceum_status()`, `refresh_lyceum_catalog()`, and `cancel_academic_refresh(job_id)`; successful capture persists through `SyncEngine` and returns a source-specific report.
- `AcademicOverview` shows separate Ciência da Computação and Engenharia de Computação trajectories; `AcademicPortal` shows independent Canvas/Lyceum status, per-section completeness, visible login state, progress, cancel, and “Atualizar tudo” actions.
- Every displayed grade/attendance/workload/result carries source label, program/term, observation date, and an unknown/discrepancy badge where needed.
- Aula links associated notes/materials from the archive catalog without copying the original file.
- Sensitive student identifiers such as RA/CPF remain local-only and never appear in generated JSON records.

- [ ] **Step 1: Add mapping tests** that compare current local register facts (seven active 2026.2 subjects, listed grade/frequency rows, prior-course summaries, and the separate Engineering trajectory) with typed database values and preserve the unknowns/discrepancies.
- [ ] **Step 2: Implement `academic_overview` and `academic_programs`** by reading the normalized local SQLite entities, not calculating official values in React.
- [ ] **Step 3: Build the Portuguese academic overview and browser refresh controls** with source details, separate Canvas/Lyceum status, per-section coverage, cancellation, and non-blocking local data.
- [ ] **Step 4: Persist each Canvas and portal capture via `SyncEngine`** with stable source IDs; identical recaptures update provenance without duplicating a course, grade, attendance row, or material.
- [ ] **Step 5: Run Rust mapping tests and the frontend production build.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib academic::tests`

  Run: `npm run build --prefix app`

  Expected: no duplicate facts on recapture; portal unavailable state does not hide local academic history.
- [ ] **Step 6: Open the packaged app** and verify both trajectories, current courses, known grades/frequency, remaining-hour discrepancy, and Aula links with Canvas disconnected.
- [ ] **Step 7: With an authenticated app-owned browser session available, run the full Canvas and Lyceum refreshes** and compare persisted Markdown/JSON/SQLite against the visible pages. Report any unavailable sections and do not claim a full record when the source omits it.
- [ ] **Step 8: Commit the academic overview and portal UI after their focused verification passes.**

## Execution Order

This plan depends on `2026-10-09-estuda-uva-sync-git.md` Tasks 1–4 and `2026-10-09-estuda-uva-local-acervo-aula.md` Tasks 1–4. Complete this plan's Task 2 before its Canvas and portal refresh tasks. Both captures use the shared local `SyncEngine`; do not add a second catalog or sync implementation.

## Portal Reference

- The repository's academic record identifies the login entry at `https://portalaluno.uva.br/Aluno/PortalAluno` and an observed Lyceum services route at `https://sis.lyceum.com.br/AOnline3/index.html#/home/servico-list`; navigate from the authenticated portal's visible UI rather than assuming unverified direct routes.
- The existing `chromiumoxide` dependency and `browser.rs` session are the browser-automation implementation; no runtime sidecar is part of this plan.
