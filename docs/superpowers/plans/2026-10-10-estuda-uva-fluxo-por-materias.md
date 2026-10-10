# Estuda UVA: Fluxo por Matérias Implementation Plan

> Implement inline from the user's explicit request. Keep the shared checkout and all existing changes; do not branch, clean, commit, or push.

**Goal:** Make active subjects the app's main entry point, open their lessons and materials in context, present lessons inside the app, and show lesson/practice history by subject.

**Architecture:** Derive the active subject list from the local academic trajectory, then join local catalog documents only through known course/semester metadata and Canvas resources only when an authenticated session is available. Add persistent lesson-reading sessions to SQLite and expose a combined history query; render the subject-first flow through focused React components while keeping the full archive and sync controls available under data tools.

**Tech Stack:** Tauri 2, Rust, rusqlite, React, TypeScript, existing local catalog scanner, existing Canvas session and Markdown renderer.

**Spec:** `docs/superpowers/specs/2026-10-10-estuda-uva-fluxo-por-materias-design.md`

## Global Constraints

- Local subject/material browsing works without Canvas login.
- Use active enrollments from the local trajectory as the source for current subjects.
- Never silently associate a document whose subject or semester is ambiguous.
- Keep official grades/frequency separate from app reading/practice history.
- Keep existing SQLite practice-session records and migrate them without deletion.
- Preserve local document paths, Markdown content, and the existing three-way sync behavior.
- Do not add or run tests unless the user requests tests; use build and runtime checks for this implementation.
- Keep the shared working tree intact; no branch, worktree, cleanup, commit, or push.

## Review Focus

- Canvas disconnected while local enrollment data exists: current subjects must still render.
- Course metadata differs in spelling or semester: do not attach the document to the wrong active subject.
- Empty or partially indexed archive: distinguish “still loading”, “not indexed”, and “no associated materials”.
- Existing practice rows from schema v1: remain readable and appear in history after migration.
- Reopening an unfinished lesson: resume the existing session; reopening a completed lesson starts a new session.
- Markdown with no headings, one heading, tables, or local images: lesson presentation remains readable and progress remains valid.

---

### Task 1: Add local lesson session and combined history storage

**Files:**
- Modify: `app/src-tauri/src/storage.rs`
- Modify: `app/src-tauri/src/lib.rs`

**Interfaces:**
- `begin_lesson_session(resource_path, course_id, course_title, material_title, section_count) -> LessonSession`: resume the latest incomplete session for the same path or create a new one.
- `save_lesson_progress(session_id, section_index, section_count, completed) -> LessonSession`: persist progress and completion time.
- `list_study_history(course_title: Option<String>, limit: usize) -> Vec<StudyHistoryEntry>`: return newest lesson and practice events with a common type.
- Tauri commands: `begin_lesson_session`, `save_lesson_progress`, and `list_study_history`.

- [ ] Add schema version 2 with `lesson_sessions`; preserve the existing `study_sessions` table and its rows.
- [ ] Implement resumption by canonical local relative path and incomplete status; completed sessions are immutable history entries.
- [ ] Return lesson and practice history in a bounded newest-first query, including subject, title, timestamp, progress, and practice score when present.
- [ ] Run `cargo check --manifest-path app/src-tauri/Cargo.toml --bin estuda-uva`.

### Task 2: Replace the archive-first landing flow with local subject browsing

**Files:**
- Create: `app/src/ui/studyTypes.ts`
- Create: `app/src/ui/StudyHome.tsx`
- Create: `app/src/ui/SubjectList.tsx`
- Create: `app/src/ui/SubjectDetail.tsx`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/styles.css`

**Interfaces:**
- `StudyHome` receives active enrollments, safe associated-document counts, recent study item, and local/Canvas loading state.
- `SubjectList` receives trajectories and the local document index; it groups active and past terms without mixing them.
- `SubjectDetail` receives one enrollment, exact/confirmed local documents, matched Canvas course/modules when available, and callbacks to open a lesson or update Canvas.
- The `LocalDocument` frontend type includes the scanner's `course`, `semester`, `category`, `readStatus`, and `readReason` metadata.

- [ ] Show the local current trajectory's enrolled subjects when Canvas is disconnected.
- [ ] Associate files only on normalized exact course title plus compatible explicit semester; present ambiguous/missing metadata for review.
- [ ] Add subject detail with separate lessons, materials, academic results, and history sections.
- [ ] Move the generic folder/file catalog, global search, conflicts, Git state, and repository sync controls under “Dados e sincronização”.
- [ ] Keep the visible Canvas login state as an update status, not as the empty state for local subjects.
- [ ] Run `npm run build --prefix app`.

### Task 3: Present lessons in sections and expose the study history

**Files:**
- Create: `app/src/ui/LessonPresenter.tsx`
- Create: `app/src/ui/StudyHistory.tsx`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/styles.css`

**Interfaces:**
- `LessonPresenter` receives a local document or imported material, its Markdown, course title, persisted session progress, and progress callbacks.
- It renders a heading outline and one readable section at a time, with previous/next, resume position, source/provenance, and optional practice after reading.
- `StudyHistory` receives combined `StudyHistoryEntry` items and filters by selected subject/term.

- [ ] Open local Markdown lessons and imported Canvas lessons in the same presenter without exposing filesystem paths as the primary content.
- [ ] Split Markdown by headings; treat heading-free text as one section and preserve GFM tables, safe links, and local images.
- [ ] Save each section transition and mark the session complete only when the learner completes the final section.
- [ ] Show reading sessions and practice results in subject context, separated from official grades.
- [ ] Add an all-subject history screen and subject-level history section.
- [ ] Run Rust and frontend builds.

### Task 4: Verify the real subject-first app flow

**Files:**
- Modify only any implementation files needed to fix issues found in runtime review.

- [ ] Build and open the desktop app with Canvas disconnected and confirm that active local subjects render.
- [ ] Open an active subject with a locally indexed document, present it, change sections, close/reopen it, and confirm that progress resumes.
- [ ] Confirm that the history view shows the reading session and existing practice records separately.
- [ ] Confirm that “Dados e sincronização” still opens the full archive, search, conflict controls, and Git status.
- [ ] Recheck `git status --short` and preserve all unrelated files and user changes.
