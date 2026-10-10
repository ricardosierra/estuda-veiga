# Design: Estuda UVA local archive and bidirectional sync

**Date:** 9 October 2026
**Repository:** `/Users/sierra/Documents/Faculdade`
**Application:** Estuda UVA
**Status:** Scope approved by the user on 2026-10-09; implementation authorized.

## Goal

Estuda UVA must use the academic files and Git repository that already exist in `Faculdade`, show Markdown lessons in an Aula reader, retain the academic facts already recorded in SQLite, and keep the three representations synchronized: SQLite, Markdown, and JSON. The app must not require a fresh download when a local copy already satisfies an import. It must remain useful without Canvas being signed in. Synchronization in this scope is between this repository's SQLite database and its Markdown/JSON files.

The archive includes the academic record already captured in `REGISTRO_ACADEMICO.md`, course notes and materials, previous-semester UVA source material, current Canvas courses, and the user's existing local study documents. Academic facts include courses, enrollments, grades, attendance, workload/progress, schedules, upcoming courses, and study activity. The app must distinguish official source data from estimates and locally authored notes.

## Current evidence

- The configured library and Git root is `/Users/sierra/Documents/Faculdade` on branch `main`; no Git remote is configured.
- The app currently keeps SQLite under `~/Library/Application Support/br.edu.uva.estuda/estuda-uva.sqlite3`. Its `courses` and `materials` tables are empty. The app currently does not index the existing Markdown files.
- The repository already contains 28 Markdown files, 29 curated `.txt` files under `materias/`, one `.alg` file, and an academic `extraction-report.json`. `originais/` also contains source documents and personal data that require careful handling.
- The Estuda UVA app uses Tauri 2, Rust, and React/Vite. It controls an isolated Chromium session through `chromiumoxide`; Canvas and portal browsing stay in Rust/CDP with this existing browser session, without Playwright.
- A separate academic SQLite source has not been found in the repository. The existing app database under Application Support is empty today, but must be preserved until migration to the repository-local database is verified.
- The desktop inventory shows Estuda UVA and Chromium running, but no Canvas tab or authenticated Canvas state is confirmed. Any authenticated import remains conditional on an existing signed-in app session or visible sign-in. Credentials and browser state must not be copied into logs, the app database, Markdown, JSON, or Git.

## Design decision

Use the Faculdade directory as the local archive and place the app's working database at the repository root as `.estuda-uva.sqlite3` (including its SQLite `-wal` and `-shm` companions). Keep this database out of Git. SQLite is the transactional/query store; Markdown is the human-editable lesson and record view; JSON is the structured exchange form. Each managed academic record/document has a stable ID, revision, source path, and content hash and is represented in all three stores.

For each synchronized record, updates from any one of the three representations flow to the other two. A normal update is reconciled automatically. If two or more surfaces changed since their last shared revision, the app preserves all versions, marks a conflict, and asks the user to resolve it; it never silently chooses the last writer. File creation/replacement uses atomic writes and a recoverable sync journal so interrupted operations can finish or roll back on the next launch.

The existing app database under Application Support is migrated with SQLite's backup mechanism into the repository-local database on first launch. Keep the original database intact until schema/data counts and hashes are verified. Migration is repeatable and idempotent. The selected external academic SQLite file, if the user chooses one later, is read-only as an input; accepted records are imported into the managed local store and thereafter synchronized there.

## Corpus and indexing

- Index every Markdown file below the configured repository root, preserving its relative path and folder hierarchy. Exclude `.git/`, dependency folders, generated builds (`target/`, `dist/`, `node_modules/`), runtime/browser profiles, and the app's own mirror/output directory to prevent loops.
- Include academic JSON files as structured records and searchable content. Keep application/dependency manifests and configuration JSON (such as `package.json`, lock files, Tauri config, TypeScript config, and capability definitions) outside the academic catalog and never rewrite them as academic data.
- Also index curated local `.txt`, `.alg`, PDF, DOCX, Pages, and academic PNG files where supported. Do not index credential files or raw personal-source data from `originais/` into the search corpus by default. Show excluded originals as local source references where useful; never copy their contents into mirrors or logs.
- Classify entries from their path and explicit metadata into courses, semesters, lectures/materials, academic records, planning, app documentation, or unclassified. Do not infer that a course was taken merely because a matching file exists.
- Preserve headings, tables, lists, relative links, and images when showing Markdown. Render raw HTML disabled. Any Markdown can open in the Aula reader; metadata indicates whether it is a lesson, record, or general document. For extracted text, provide a readable Aula layout without inventing missing content, grades, questions, or conclusions.
- Search uses a local index rebuilt or reconciled from the real files and database. Search results retain exact source location and link back to the source. An unreadable/unsupported item remains visible with a reason and does not suppress healthy files.

## Three-way record format and sync rules

### Records and mirrors

- Introduce a versioned sync schema with stable record IDs for documents and structured academic entities: courses, semesters, enrollments, grades, attendance, workload/progress, schedules, materials, lessons, calendar events, and study sessions.
- Store each managed record and editable text in SQLite. Produce a Markdown view and a JSON envelope for the same ID. The JSON envelope carries schema version, ID, type, source/relative path, revision, content hash, timestamps, structured fields, and the Markdown/body content needed for a complete round trip. The Markdown view contains a small machine-readable frontmatter block for ID/schema/revision plus readable content. User-visible facts cite their source file, Canvas page/record, or portal capture date where known.
- Preserve the user's established root files and paths, including `REGISTRO_ACADEMICO.md` and course notes. Use an app-owned mirror area inside the repository for generated JSON/Markdown counterparts that do not already have a natural human-maintained file. Exclude only this mirror area from source discovery; include the outputs in Git.
- Use `materias/catalogo/` for normalized, versioned curriculum and trajectory JSON (`disciplinas.json`, `relacoes.json`, per-program curriculum files, and `trajetorias.json`). Put document-level counterpart projections under `.estuda-uva/mirrors/`, preserving the original relative path and extension in the mirror name. For example, a source `notas.md` has a JSON envelope in the mirror area, and an eligible source `resumo.json` has a Markdown projection there. Never scan generated mirrors as new source files.
- Existing academic Markdown is imported as-is into SQLite and receives a structured JSON mirror. Existing supported academic JSON is parsed and imported; its Markdown counterpart is a reversible projection. Unknown JSON fields are retained exactly in the JSON payload and shown as data, not discarded. App and build configuration JSON remains outside this contract.
- Keep mirrored Markdown body text lossless. Where a Markdown document does not have a known academic structure, synchronize it as document text and do not infer grade/enrollment entities from prose. Parse structured fields only through explicit, validated adapters; keep unknown sections unchanged and surface ambiguous mappings for review.
- Peer versions carry a per-device causal vector. A peer exchanges a bounded manifest before requesting missing eligible Markdown/JSON and logical records; each item and batch has a hard size limit. TLS 1.3 pins both device keys, and pairing plus each transfer are user-started. Listeners close when the explicit operation ends.
- Existing local binary originals (PDF, DOCX, Pages, images) are never moved or deleted by indexing. SQLite stores metadata, hashes, extracted searchable text where supported, and a relative source reference; it does not duplicate those files. For newly downloaded Canvas materials, use the source bytes only in a bounded temporary location for extraction, persist the readable Markdown and provenance, then remove the temporary binary on success, error, cancellation, and recovery after restart. Unsupported binary materials remain a metadata/link entry with a clear unsupported status.

### Reconciliation

- On startup and explicit “Sincronizar agora”, scan source Markdown and eligible JSON, check the managed database, then reconcile changed records. While open, observe eligible source paths and external writes to the workspace SQLite database/WAL; a debounced watcher plus per-record `data_version`/content hashes detects changes made in another SQLite client. Watcher events are hints and are coalesced; record hashes determine actual changes, so unrelated DB writes and the app's own synchronized transactions do not echo.
- Track the last common revision/hash independently for SQLite, Markdown, and JSON. If exactly one side changed, validate and apply its update to the other representations, then record a new shared revision. If all are unchanged, do nothing. Stable hashes and sync-origin markers prevent echo loops.
- If two sides changed compatibly (disjoint structured JSON fields or separate metadata/body fields), merge them deterministically and record the merge. Otherwise create a visible conflict with links to the original versions and a preserved conflict snapshot. Resolution writes the chosen/merged result to all three surfaces as a new revision.
- A source deletion is a tombstone, not permission to erase the record or mirrors. Show the deletion and offer restore, keep-archived, or explicit delete across all managed representations. Keep recovery snapshots for an agreed retention period; never silently remove user content.
- Reject malformed JSON/frontmatter with a per-file error and leave the last valid database record and other mirror untouched. Never truncate a file on partial write. Bound file count, depth, and size, and report skipped/error entries individually.
- Manual synchronization and file watching never invoke Canvas or make network requests. They operate only on local files and the local database.

### Source attribution and official-record accuracy

- Separate portal/Canvas facts from local notes and derived estimates. A grade, attendance value, workload, and completion number must retain the source and observation date. If portal totals disagree with course-hour sums, preserve both and show the discrepancy; do not reconcile by inventing a value.
- Missing grades, attendance, prior-course transcript data, upcoming curriculum, or historical records remain explicitly unknown until a source provides them. A local note is not presented as an official transcript.
- The app may refresh Canvas information only after the user initiates a Canvas action. Prefer already-indexed/local material; compare course/resource IDs, known filenames, paths, and hashes before downloading. Skip an already-present matching item. If a candidate path exists with different content, preserve it and offer a distinct filename/cancel choice; never overwrite hand-edited files.

### Academic identity model

- Keep institution, program, curriculum version, official curricular offering, canonical subject, and the student's trajectory as separate entities. A course in Canvas is not automatically the same thing as a curriculum offering.
- An official offering is identified by institution, program, curriculum version, and official course code. A canonical subject can link multiple offerings for shared materials without combining their enrollments, grades, attendance, workload, or completion.
- Store relationships as sourced `same_subject`, `partial_overlap`, `equivalence_candidate`, or `equivalence_confirmed`. Similar names/hours can flag a candidate; only explicit institutional evidence can confirm equivalence or transfer completion.
- Prior and current program results remain attached to their respective trajectories. Aggregate hours without per-course evidence stay aggregate. Preserve source discrepancies: Ciência da Computação currently has a 592 h gap between the portal's stated remaining hours and the pending-course list, plus a separate 176 h residual after adding in-progress courses; Engenharia de Computação has a distinct 180 h difference between stated remaining hours and its pending-course list. Do not reconcile any of these by inventing hours.
- Detailed field definitions and stable-ID rules are in the companion `docs/superpowers/specs/2026-10-09-catalogo-academico-sincronizacao-design.md`.

## User experience

### Overview

- Show the configured repository path, local sync status, last completed synchronization, conflict/error count, and Git working-tree status separately from Canvas status.
- Show courses and semesters with provenance, grades and attendance where recorded, completed/in-progress/remaining workload, upcoming/required courses where the curriculum source supports them, and a clear unknown/discrepancy state.
- A Canvas disconnected/expired/login status is a Canvas-only badge. It never blocks the local catalog or displays an empty local archive as if sign-in were required.

### Acervo and Aula

- Reproduce the on-disk academic folder tree and list Markdown, supported local materials, eligible JSON records, and relevant originals. Keep app/source-code documentation discoverable in its own category, not mixed into course lessons.
- Search document names and extracted/indexed text by course, semester, category, and type. Open every Markdown item in an Aula-style reader with a heading outline, source path, course/semester labels where known, links to related PDF/original, and edit/sync state.
- Render eligible JSON records as structured fields and show an escaped raw-data view. Unknown fields remain available. Offer editing only when a reversible adapter exists; otherwise mark that view read-only and keep the JSON itself untouched.
- A “Conflitos” view compares SQLite, Markdown, and JSON revisions and offers merge/choose/restore actions. A “Git” view reports changed, untracked, and ignored app data using the local repository status.

### Canvas and import

- Keep a visible, persistent, app-isolated browser profile controlled by the existing `chromiumoxide` Rust/CDP session. Do not add Playwright, WebDriver, Selenium, or Node browser automation. Sign-in and MFA stay visible; do not copy cookies or profile data from the user's personal Chrome profile. A user-started “Atualizar Canvas” action enumerates every accessible course, module, page, assignment, announcement, file, and available grade/activity view, including pagination, and reports inaccessible sections instead of implying completeness.
- Import all accessible supported course materials into the existing archive layout as Markdown with course, module, source URL (sanitized), capture time, and page/source provenance. A batch is cancellable and reports progress and per-item results. PDF bytes are temporary inputs for conversion and are deleted after extraction; existing same-source local material is reused. Unsupported binaries are listed with metadata and a usable source link without retaining a local copy.
- Before network transfer, reconcile local file paths and stored Canvas resource IDs, known metadata, and available hashes to avoid downloading an item already represented locally. Network import remains separate from background local sync; no Canvas access occurs on polling or filesystem-watch events.

### UVA academic portal and transcript

- Provide an explicit, read-focused action to open the UVA/Lyceum academic portal in a separate visible page of the existing isolated `chromiumoxide` Rust/CDP session. Use the installed Chrome/Chromium executable when available and keep its profile app-isolated.
- The official [Portal do Aluno UVA](https://portalaluno.uva.br/Login) currently presents its visible login route when unauthenticated. Continue reading only after the student signs in through that visible portal session.
- Navigate all available read-only portal tabs/sections, follow pagination, and capture the student's profile identifiers only when safe for local display, transcript, course history, grades, attendance, curriculum/progress, schedules, and available upcoming-course data. Preserve source context; each captured value records page/source, capture date, academic program, and term when available. Never claim a section was read if it was inaccessible.
- Never infer unseen pages or convert a summary total into course-level grades. If a portal page requires login, keep it visible for user sign-in; any approved credential note is consumed without showing or logging its contents and is never synced to academic records.
- Portal access is read-only: do not submit enrollment, withdrawal, equivalency, financial, or profile changes. Save captured academic facts into the SQLite/Markdown/JSON sync contract with provenance and show errors or inaccessible sections explicitly.

## Git and repository integration

- Reuse the existing Git repository and current branch. Do not run `git init`, create a branch/worktree, configure a remote, push, or publish.
- Track human-readable Markdown, eligible JSON mirrors, root `AGENTS.md`/`CLAUDE.md`, and ordinary app source changes. Show the exact diff and leave changes unstaged and uncommitted for the user; preserve unrelated working-tree changes.
- Ignore `.estuda-uva.sqlite3`, its `-wal`/`-shm` files, lock/journal/temp files, browser profile/session, credential files, downloaded caches, and build outputs. Never add a password, cookie, token, private browser data, or raw personal source to Git.
- Use `git status --short` after synchronization to show repository effects; do not revert or clean unrelated user changes.

## Synchronization boundary

- SQLite, Markdown, and JSON reconcile locally inside the configured Faculdade repository on startup, explicit sync, and watched source changes. These local operations do not contact Canvas, Lyceum, or another computer.
- The user may explicitly pair Estuda UVA installations and synchronize eligible academic records and Markdown/JSON directly over a reachable LAN or VPN connection. Each installation stays independently usable offline; there is no central server, cloud relay, or always-on listener.
- A peer receives logical records and eligible text files through the shared conflict/tombstone rules. It never receives the live SQLite file/WAL, machine settings, credentials, cookies, browser profile, temporary PDF downloads, or excluded raw originals.

## AGENTS.md and CLAUDE.md bootstrap

- On app setup, check for root `AGENTS.md` and `CLAUDE.md`. If either is missing, create only that missing file with a concise project-specific template explaining the archive layout, sync contract, database ignore rules, no-redownload behavior, safe source handling, and verification commands.
- Use exclusive create/atomic rename semantics so an existing file, symlink, or concurrently created file is never overwritten. If the file exists, preserve it byte-for-byte and do not append or “repair” it automatically.
- The templates contain no credentials or personal academic details. They are ordinary Markdown tracked by the existing repository.

## SQLite and migration

- Open the repository-local database with WAL mode and schema migrations. Use transactions, foreign keys, prepared statements, bounded queries, and a sync journal/outbox for filesystem projections.
- First launch copies the existing Application Support database using SQLite backup, validates schema/table counts and a content-independent integrity check, then opens the repository-local copy. Do not delete the old database automatically. If migration validation fails, retain both and show a recovery action.
- Extend existing SQLite tables without changing their meaning: Canvas course/material identities remain distinct from institutional curricula and offerings. Keep academic events and study sessions in the synchronized academic model; machine-specific preferences, browser state, Canvas cookies/IDs, and runtime cache remain local-only unless explicitly represented as user-owned study data.
- Keep app settings needed to locate the archive and Chromium profile in their existing platform-appropriate application-data location if they contain machine-specific paths or browser state; the synced academic database itself lives in the repository.
- Do not store credentials in SQLite, Markdown, JSON, logs, or Git. A credential note, if securely identified, is used only for the explicitly authorized, visible Canvas or read-only UVA/Lyceum session and is never imported into the app's sync corpus.

## Security and data safety

- All local reads/writes stay in Rust commands. Canonicalize paths under the configured root; reject traversal and symlink escapes. The webview cannot request arbitrary local paths.
- Never execute document HTML, scripts, macros, or remote resources. Render Markdown with raw HTML disabled; render HTML as sanitized/extracted text. Mask password/token-like academic SQLite fields and never log record contents.
- File content and academic records stay on the paired computers. Do not introduce LLM, telemetry, or a third-party indexer. Only user-started Canvas/portal synchronization may contact UVA services. Canvas downloads keep HTTPS, host/redirect/signature, and 60 MiB checks; downloaded source binaries are short-lived extraction inputs only.
- Preserve unknown fields and unclassified records rather than dropping them. Never claim an inferred or estimated academic fact is official.

## Error and status model

- Local archive: scanning, synchronized, changed/pending, conflict, partial (per-file errors), inaccessible, or empty only when the bounded scan found no supported files.
- SQLite: ready, migration/recovery needed, unavailable, or schema error. A failed mirror write leaves a recoverable journal entry and a visible pending status.
- Git: clean, changed/untracked, ignored local database present, or repository error. Git errors never block local reading/sync.
- Canvas: disconnected, waiting for visible login, authenticated, expired, browser error, or per-file import result. Canvas state does not control local status.
- Explain which file/record failed and a next step; one malformed item must not blank the course overview.

## Acceptance criteria

1. On startup with the configured Faculdade root and an empty Canvas database, the app opens the local archive and displays existing academic Markdown/materials without requiring Canvas.
2. All in-scope Markdown files are findable by path/title and readable in Aula view; existing course/semester grouping works, and app/project documentation remains identifiable as such.
3. Existing academic JSON files are discoverable, parsed without dropping unknown fields, and searchable. App/dependency configuration JSON is not rewritten or shown as academic data.
4. The managed database is at the repository root; first-launch migration preserves the old Application Support database and produces a verifiable repository-local database.
5. Editing one managed academic record in SQLite, its Markdown projection, or its JSON envelope updates the other two and survives closing/reopening the app.
6. Edits made externally to Markdown or JSON are reconciled on refresh/watcher event, with stable IDs and no duplicate records or echo loops.
7. Simultaneous divergent edits produce a visible conflict and preserve each version. Malformed input or an interrupted process never destroys the last valid version; restart resumes or reports the journaled operation.
8. Deleting a source file does not silently delete its database record/mirrors; user-visible tombstone resolution can restore or explicitly remove it.
9. `AGENTS.md` and `CLAUDE.md` are created independently only when missing; pre-existing files remain byte-for-byte unchanged.
10. SQLite/WAL/SHM, credentials, browser profile/session, and build/runtime data are ignored by Git. Markdown/JSON mirrors and bootstrap guidance appear in status as normal local repository changes; no automatic stage/commit/push occurs.
11. Existing local Canvas resources are identified and reused before download. A different file at a candidate path is never overwritten. A user-started complete Canvas refresh visits all accessible courses and material sections; supported downloads become searchable Markdown, with PDF bytes removed from temporary storage after conversion or any failed/cancelled attempt.
12. Canvas disconnect, network failure, or an unavailable app database never hides local source files; errors are reported per item.
13. All local paths are bounded to the configured repository. No passwords, cookies, tokens, private browser state, or academic content are emitted to logs or sent to a third party.
14. Academic grades, attendance, completion, workload, and upcoming-course claims show source/date and preserve unknowns and discrepancies without inventing official values.
15. The UVA/Lyceum portal and Canvas are browsed through the existing isolated Chromium session from Rust/CDP, with no Playwright; all available read-only sections/pages are traversed with per-section completeness and provenance, and no portal mutations occur.

## Verification plan

Use a temporary fixture repository containing Markdown with links/tables/images, recognized and unknown academic JSON, config JSON, TXT/ALG, a small PDF, `AGENTS.md`/`CLAUDE.md` present/missing variants, a SQLite database, and Git. Verify startup/indexing, each direction of the three-way round trip, no-op hash idempotence, watcher echo suppression, disjoint merge, divergent conflict preservation, malformed file safety, deletion tombstones, crash/outbox recovery, migration repeatability, credentials/path exclusions, and Git status/ignore behavior. Verify the real library's existing files without rewriting or downloading them. Then run Rust tests, frontend build, formatter, packaged-app smoke checks, and inspect Git diff/status; distinguish those results from live authenticated Canvas evidence.

## Known limitation before implementation

The transcript in `REGISTRO_ACADEMICO.md` records an authenticated Portal do Aluno capture dated 9 October 2026. If a later refresh requires authentication, keep sign-in visible in the app-isolated Chromium page and use only the credential note the user identified in the Faculdade folder; never print it, log it, place it in app data, or synchronize it. Local catalog/sync work does not depend on login.

## Implementation references

- Existing Estuda UVA design: `docs/superpowers/specs/2026-10-08-estuda-uva-design.md`.
- Existing archive design: `docs/superpowers/specs/2026-10-08-acervo-academico-design.md`.
- Existing Canvas session, download/conversion, archive, and storage: `app/src-tauri/src/browser.rs`, `app/src-tauri/src/download.rs`, `app/src-tauri/src/pdf.rs`, `app/src-tauri/src/archive.rs`, `app/src-tauri/src/storage.rs`, and `app/src-tauri/src/lib.rs`.
- Existing Tauri views: `app/src/ui/App.tsx` and `app/src/ui/styles.css`.
