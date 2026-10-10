# Design: Estuda UVA local academic catalog

**Date:** 9 October 2026
**Repository:** /Users/sierra/Documents/Faculdade
**Application:** Estuda UVA
**Decision approved in chat:** Add a local, searchable catalog for the configured academic folder and keep it usable without a Canvas login.

## User goal and current evidence

The student wants the Monitor to show and open the material that is already saved for the current courses, and to consult the academic information from inside Estuda UVA without first signing in to Canvas.

The live application currently points to /Users/sierra/Documents/Faculdade. Its local SQLite database has zero rows in courses and materials, while the folder contains the academic files. The current overview and Acervo read only that database. Canvas login status is also presented next to the empty-library message, which makes an online-session state look like a failure of local reading.

The checked-in course map identifies Bases de Programação and Paradigmas de Linguagens de Programação as the two courses currently available in Canvas for 2026.2. The academic folder also has prior-semester material, local notes, course and materials registers, study records, text files, Word originals, a Pages original, and UML diagrams.

## Scope

The catalog reads academic documents under the configured library root, including current and previous semesters. Its academic areas are recognized files directly under the root (registers, calendars, plans, and study notes), docs/ except docs/superpowers/, materias/ recursively (including the 2026.1 UVA source archive), and originais/ recursively. The directory walk prunes repository internals and application/build data before enumeration, including .git/, .superpowers/, docs/superpowers/, app/, node_modules/, dist/, target/, and tmp/.

The catalog includes known readable academic formats and keeps other regular files inside those academic areas visible as unsupported entries with path, type, and reason. It ignores operating-system noise such as .DS_Store. It does not enumerate source code, dependencies, or build output as academic material.

The catalog is read-only. It never copies, renames, moves, edits, or deletes a source file. It does not transmit file names or contents over the network. Canvas remains a separate, optional source for online data.

Supported inputs reflect the files present in this library:

- Markdown, plain text, HTML, VisuAlg source, and JSON metadata.
- PDF text, extracted locally and in memory with the existing Rust PDF library; scanned pages are reported as visual-only because OCR is not part of this design.
- DOCX text, paragraphs, and tables.
- Pages text when the local parser can recover it; source files remain untouched.
- PNG diagrams stored inside academic materia folders, displayed through the app's controlled local-file command.

An unsupported or unreadable item remains visible with its path, type, and a clear explanation. It is never counted as readable content or silently dropped. This first delivery does not add support for other office formats that are not present in the library.

## Approaches considered

1. **Rebuild the current SQLite import tables from files.** This would make the existing counters nonzero, but conflates locally authored documents with PDFs imported from Canvas and can become stale when a file changes. It does not give the student a useful way to search and read the whole archive.
2. **Read-only local catalog with refresh and in-app search — recommended.** Scan the configured academic root, expose recognized files as document entries, and read a selected document on demand. Counts and results reflect files actually present. Canvas-specific data remains in its existing database and session.
3. **Copy or import every file into SQLite.** This creates duplicate authoritative copies, consumes space, complicates personal-document privacy, and needs conflict rules for hand edits. It does not fit the existing local archive contract.

The selected approach is 2. SQLite may retain the app's existing Canvas and study data. The local catalog is derived from the filesystem and must not require a migration or a pre-populated database.

## User experience

### Overview

- Show the configured library path and a local scan result as separate state from Canvas.
- Show the number of readable academic documents and the number of local courses/semesters represented by those documents.
- For study-session or accuracy data, distinguish “no records yet” from an error or an unavailable Canvas session.
- A disconnected or waiting-for-login Canvas status remains visible as a Canvas-only badge and action. It does not replace local counts, block Acervo, or produce an app-wide error banner.
- The first view must show discovered local content even when Canvas is disconnected, expired, unavailable, or never opened.

### Acervo

- List academic documents found on disk, including files in materias/ and local originals/.
- Provide text search over extracted contents and file titles, with filters for course, semester, and file type. Search covers the root registers, both 2026.2 courses, prior-semester materials (including `materias/fontes-uva/2026.1`), local originals, and general UVA academic guidance.
- Group the two current courses under Bases de Programação and Paradigmas de Linguagens de Programação. Keep previous semesters and general academic records accessible through the same catalog.
- Open a selected document inside the app. Markdown renders with raw HTML disabled. Plain text, source, and JSON render as escaped text. HTML renders extracted safe content, not executable page markup. DOCX and Pages render extracted text, headings, and tables to the extent supported by the reader. Academic PNG diagrams render as images.
- Resolve relative links to another file within the configured root as in-app navigation. Keep external official-source links visibly distinct.
- Report unsupported content and per-file read errors beside the affected item so one bad file does not blank the catalog.

### Refresh

- Scan when the application starts and when the student invokes “Atualizar dados locais”. Build the searchable text index in process memory from the bounded scan results; clear and rebuild it on refresh and never persist document contents in SQLite or another cache.
- Do not rescan every three-second Canvas status poll.
- A refresh reflects added, edited, or removed files on the next scan. It never modifies those files.
- Changing the configured root affects subsequent scans only after it passes existing-directory validation.

## Data flow and boundaries

1. Rust canonicalizes the configured root and enumerates academic files, excluding the named application/build directories.
2. The walker rejects paths outside the root, skips symlinks that escape the root, bounds recursion and per-file size, and records read errors per entry.
3. Rust derives course and semester labels from existing folder names and the academic course/material registers. It does not infer official enrollment from the mere existence of a document.
4. Tauri commands return document metadata and scan status, search the in-process text index, and read the selected document for display.
5. The frontend renders returned data only. It does not read arbitrary local paths or calculate authoritative course counts.
6. Existing SQLite Canvas course records, material imports, calendar events, and study sessions remain separate. Existing imported Markdown is represented once by its physical file and may retain its existing “open guided lesson” action.
7. The catalog code makes no network requests. Only explicit Canvas actions use the existing Chromium session.

Do not add a persistent content cache. Any in-process extraction cache is discarded on refresh and app exit; it is keyed by canonical path, size, and modification time, and preserves the read-only source contract.

## File parsing and safety

- Keep filesystem access in Rust. The webview receives structured metadata and the contents of the file the student selected.
- Resolve each requested path relative to the configured root, canonicalize it, and reject traversal or symlink escapes.
- Apply a documented maximum input size and a bounded file count/depth. An oversized or unreadable file gets its own status and does not interrupt other entries.
- Never execute HTML, JavaScript, macros, external resources, or content from a document. Render HTML as sanitized/extracted text. Render Markdown with raw HTML disabled.
- Extract PDF text during the bounded local indexing pass, retain page boundaries where the existing extractor provides them, and state when there is no selectable text. Do not persist downloaded or copied PDFs.
- Parse DOCX and Pages from read-only bytes. Preserve paragraph order, tables, and text links where supported. Do not save a converted or modified source document.
- Load images only from approved academic catalog entries inside the configured root; do not expose a general arbitrary-file URI to the webview.
- File content and document metadata remain on this computer. No AI, telemetry, upload, or indexing service is introduced.

Potential Rust parser candidates need a small read-only spike against the actual local originals before they are adopted. The docs.rs documentation for iwork exposes Pages text storages, and docx-lite describes text and table extraction. Pin selected versions and prove the existing DOCX and Pages originals decode in order; if a candidate cannot read them, choose another local parser before release rather than shipping the known originals as unsupported.

## Error and status model

- Local catalog: scanning, ready with counts, empty with an actual supported-file count of zero, inaccessible root, or partial with per-file errors.
- Canvas: disconnected, waiting for manual login, authenticated, session expired, browser error.
- Canvas session messages are not local catalog errors.
- Local read failures identify the affected document and a practical next step; do not replace the whole overview with zeros.
- A missing Canvas session does not stop search, preview, or opening local course documents.

## Acceptance criteria

1. On startup with the configured Faculdade root and an empty Canvas database, the overview reports a successful local scan and nonzero discovered academic documents.
2. The catalog shows materials belonging to both Bases de Programação and Paradigmas de Linguagens de Programação without Canvas authentication.
3. Search finds known text from each course's existing local material and opens the selected source in the app.
4. Root academic indexes, current-semester notes/materials, the 2026.1 UVA source archive, local originals, general academic guidance, and UML diagrams are accessible and correctly labeled. The two current courses are not the entire catalog.
5. Existing DOCX and Pages files are readable in-app through local Rust parsing, validated against the real files before release. Paragraph order and readable table text are preserved; parsing limitations are explicit.
6. PDFs with selectable text can be searched/read without creating a PDF copy. Image-only PDFs receive an explicit no-text/OCR-not-supported state.
7. Adding, changing, or removing a supported file is reflected by the next explicit refresh. Source files remain byte-for-byte untouched.
8. The local scan excludes application/build internals and reports scan errors per file without suppressing healthy documents.
9. An inaccessible or unauthenticated Canvas session leaves local catalog use fully functional. Online course/calendar/import actions continue to require their current Canvas session behavior.
10. A path-traversal or symlink escape cannot read outside the configured root. File content is never sent over the network.
11. No local-document count is sourced from the Canvas SQLite courses/materials tables. Those tables continue to represent Canvas/app-managed state only.
12. The packaged Estuda UVA app is opened with Canvas disconnected and visibly demonstrates both course folders, a successful query from each, and a selected document open inside the app.

## Risks and responses

| Risk | Response |
|---|---|
| DOCX or Pages reader loses structure | Validate real local sources and fixtures; preserve source files; show extraction limits; do not claim faithful visual layout |
| Academic registers use varied naming | Use explicit folder mapping plus current register files; unknown association is “geral/não classificado”, not an invented course |
| Build artifacts inflate the scan | Exclude known repository/application/build roots before reading; restrict file types; bound file count and depth |
| A user edits a file while the app is open | Rescan on explicit refresh, discard the old in-memory index, and read the selected file for display |
| Local documents contain personal information | Keep access local, read-only, and behind root-bound backend commands; do not log contents |
| Canvas remains unauthenticated | Keep Canvas status visible but scoped to Canvas-only actions; local catalog starts independently |

## Implementation references

- Existing app design: docs/superpowers/specs/2026-10-08-estuda-uva-design.md.
- Existing archive design: docs/superpowers/specs/2026-10-08-acervo-academico-design.md.
- Existing Canvas and study persistence: app/src-tauri/src/storage.rs and app/src-tauri/src/lib.rs.
- Existing Tauri views: app/src/ui/App.tsx and app/src/ui/styles.css.
- iwork Rust documentation: https://docs.rs/iwork/latest/iwork/ — read Pages documents and text storages.
- docx-lite Rust documentation: https://docs.rs/docx-lite/latest/docx_lite/ — extract DOCX text, paragraphs, and tables.
