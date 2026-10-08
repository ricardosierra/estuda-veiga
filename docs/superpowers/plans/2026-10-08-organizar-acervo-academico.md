# Academic Archive Organization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task.

**Goal:** Organize academic materials by subject, place readable text in Git history, preserve editable personal-submission sources locally, and keep UVA course PDFs as official links rather than local copies.

**Architecture:** Preserve the existing semester registry (`README.md`, `CURSOS_2026.1.md`, `MATERIAIS_2026.1.md`, and `PLANEJAMENTO.md`). Add a `materias/` tree for searchable, privacy-safe academic text; keep editable DOCX/TXT/Pages personal-submission sources under ignored `originais/`; keep UVA course-material PDFs as official links and write compact notes in Markdown. Track only the three small UML diagrams as binary assets.

**Tech Stack:** Git, Pandoc (DOCX to GFM Markdown), Poppler `pdftotext` (PDF text extraction), shell and Python standard library for normalization and inventory checks.

**Spec:** `docs/superpowers/specs/2026-10-08-acervo-academico-design.md`

## Decisão posterior sobre PDFs das disciplinas UVA — 2026-10-08

O estudante decidiu manter somente notas autorais em texto e links oficiais para os materiais das disciplinas UVA. Esta decisão substitui as etapas abaixo que pedem cópias locais ou extração integral de PDFs de aula. A revisão dos 75 itens de módulos encontrou 34 PDFs únicos: 16 materiais de unidade, 10 documentos de apoio, 2 documentos institucionais e 6 PDFs anexados a trabalhos enviados. A nota para LPA U2 foi atualizada com o PDF oficial. As notas das 20 unidades permanecem em `NOTAS_DE_ESTUDO_2026.1.md`; elas resumem conteúdo acessível no Canvas, não transcrevem os PDFs. Uma instrução posterior ampliou a preferência por texto: os PDFs locais de Bases AVA 2 e Paradigmas AVA 2 foram comparados aos DOCX (similaridade normalizada 1,000) e movidos para a Lixeira, onde podem ser recuperados; os DOCX/TXT/Pages editáveis permanecem em `originais/`.

As verificações de conclusão deste plano devem confirmar que não há PDFs de disciplina e apoio no repositório, que os links estejam indexados e que as limitações de leitura estejam registradas sem inventar conteúdo. As páginas HTML das unidades exibem textos, tópicos e exemplos que sustentam resumos autorais. Cópias locais de Desenvolvimento Web U1 e U2 tiveram texto extraído para conferir as notas; depois da extração, foram movidas para a Lixeira. PDFs e texto bruto não foram adicionados ao repositório. Uma nova tentativa de obter U1 pelo link do Canvas expirou; foi usada a cópia que já estava em Downloads. Os outros 14 PDFs de unidade, 10 documentos de apoio, 2 documentos institucionais e 6 anexos de trabalhos ainda não tiveram o texto integral extraído. A prévia visual do PDF U2 permitiu observar as páginas 12–13, e a prévia de um PDF de critérios ficou preta. A antiga Task 3 de extração integral U1 fica supersedida por esta atualização.

## Global Constraints

- Keep all existing 2026.1 course, planning, and materials documents.
- Keep original DOCX, Pages, and preexisting text files for personal submissions out of Git under `originais/`; move redundant personal PDFs to Trash after verifying editable equivalents; do not keep local copies of UVA course-material PDFs.
- Do not add a full student name or matrícula to tracked files; create redacted text derivatives and preserve full sources locally.
- Keep the three UML PNG files tracked and referenced from the Paradigmas AVA 2 document.
- Keep official course-material links in the inventory; label notes as authorial summaries of accessible Canvas HTML, not as full PDF extractions.
- Preserve the text-alternative Paradigmas AVA 2 file as a distinct version because it differs from the DOCX/PDF.
- Work in the current local repository on `main`; no remote is configured.

## Review Focus

- **DOCX structure:** Pandoc preserves paragraphs but formats numbered section labels as bold text; verify they become Markdown headings and body text remains in source order in Task 2.
- **Personal identifiers:** Author name and matrícula appear in document covers and algorithm comments; verify neither occurs in tracked subject files in Task 2 and the final audit.
- **Alternate text version:** Existing Paradigmas AVA 2 text differs from the final DOCX/PDF; verify it remains a separate, clearly labeled derivative in Task 2.
- **Ignored and tracked images:** `.gitignore` excludes all PNGs by default; verify only the three UML diagrams are re-included and ordinary binary sources remain ignored in Task 1 and Task 4.
- **Course-note coverage:** Verify all 20 Canvas units have a text note and that notes point to official source pages; disclose that they are authorial Canvas-based summaries, not full PDF transcriptions, in Task 3.
- **PDF inventory:** Verify the 34 unique PDFs are represented by category and that six Canvas submission PDFs use assignment-page links rather than private download URLs.

---

### Task 1: Integrate the archive with the existing semester registry

**Files:**
- Modify: `.gitignore`
- Modify: `README.md`
- Modify: `docs/superpowers/specs/2026-10-08-acervo-academico-design.md`
- Create: `materias/README.md`

**Interfaces:**
- Consumes: The existing course and download inventories in `CURSOS_2026.1.md` and `MATERIAIS_2026.1.md`.
- Produces: Root navigation to the course archive, subject-text tree, ignored original sources, and the three tracked UML diagrams.

- [x] **Step 1: Ignore original sources while allowing the UML diagrams**

Add `/originais/` to `.gitignore`. Retain the existing PDF, DOCX, Pages, and media rules. Add precise negation rules for `materias/paradigmas-de-programacao/diagramas/diag_classes.png`, `diag_estados.png`, and `diag_sequencia.png` only.

- [x] **Step 2: Add the subject archive entry point**

Create `materias/README.md` listing Bases de Programação, Desenvolvimento de Aplicações para Web, and Paradigmas de Programação, with links to the deliverables added by Tasks 2 and 3. Add `materias/` and `originais/` to the existing root README without removing current semester references or its no-personal-identifiers rule.

- [x] **Step 3: Update the design record for the current repository state**

Record that existing semester files remain authoritative, course PDFs stay as official links only, and tracked derivatives omit personal identifiers.

- [x] **Step 4: Verify ignore behavior and links**

Run `git check-ignore -v` on representative DOCX/PDF/Pages originals under `originais/`; each must be ignored. Run `git check-ignore` on the three future UML paths; none may be ignored. Confirm every link in the new `materias/README.md` points to a planned path.

- [x] **Step 5: Check and commit the registry integration**

Run `git diff --check`, inspect `git status --short`, then commit only `.gitignore`, `README.md`, the updated spec, and `materias/README.md`.

### Task 2: Convert the existing academic submissions and preserve their sources

**Files:**
- Create: `materias/bases-de-programacao/ava1-apuracao-de-votos.md`
- Create: `materias/bases-de-programacao/ava2-monitoramento-de-esteira.md`
- Create: `materias/bases-de-programacao/codigo/monitoramento-de-esteira.alg`
- Create: `materias/paradigmas-de-programacao/ava1-caso-de-uso-saque.md`
- Create: `materias/paradigmas-de-programacao/ava2-modelagem-uml-biblioteca.md`
- Create: `materias/paradigmas-de-programacao/versoes/modelagem-uml-texto-alternativo.md`
- Create: `materias/paradigmas-de-programacao/diagramas/diag_classes.png`
- Create: `materias/paradigmas-de-programacao/diagramas/diag_estados.png`
- Create: `materias/paradigmas-de-programacao/diagramas/diag_sequencia.png`
- Move: four root DOCX files, two matching root PDF files, and the two existing `.txt` files into subject folders under `originais/`
- Move: `../trabalho-paradigmas-ricardo.pages` into `originais/paradigmas-de-programacao/`
- Move: three files from `diagramas_uml/` to the tracked subject diagram directory

**Interfaces:**
- Consumes: The existing root DOCX/PDF pairs, the algorithm text, the alternate Paradigmas text, and UML diagrams.
- Produces: Four searchable redacted Markdown submissions, one redacted `.alg` source, one separately labeled alternate-text Markdown file, and three referenced UML figures.

- [x] **Step 1: Convert the four DOCX files to GFM Markdown**

Use Pandoc with `-t gfm` for the four documents. Promote bold numbered section labels to `##` headings, retain the contents in order, and remove the full student name and matrícula from every tracked derivative.

- [x] **Step 2: Preserve the two different text inputs**

Sanitize the Bases AVA 2 code text into `monitoramento-de-esteira.alg`. Convert the distinct Paradigmas AVA 2 `.txt` to Markdown, label it as an alternate text version without claiming which source is newer, and redact the same personal identifiers.

- [x] **Step 3: Add the UML figures and captions**

Move the three existing PNGs without re-encoding them. Reference each image from the matching figure caption in `ava2-modelagem-uml-biblioteca.md` and preserve the captions and source notes.

- [x] **Step 4: Move complete originals into the ignored source archive**

Create `originais/bases-de-programacao/` and `originais/paradigmas-de-programacao/`. Move source DOCX/PDF/TXT files and the matching `.pages` document there. Keep the `.pages` and DOCX originals distinct because their cover pages use different discipline titles.

- [x] **Step 5: Verify conversions and privacy before staging**

Compare normalized extracted text for the two PDF/DOCX pairs. Search the new `materias/` tree for the known student name and matrícula; both searches must return no matches. Open the Markdown files and verify section headings, code fences, captions, and image links.

- [x] **Step 6: Confirm original files remain local and commit the subject materials**

Use `git check-ignore -v` for representative paths under `originais/`. Confirm all moved originals exist and have nonzero sizes, then commit only the new Markdown, `.alg`, and UML PNG files.

### Task 3: Create compact text notes for the UVA course units

**Files:**
- Create: `NOTAS_DE_ESTUDO_2026.1.md`
- Modify: `README.md`
- Modify: `MATERIAIS_2026.1.md`
- Modify: `PLANEJAMENTO.md`

**Interfaces:**
- Consumes: The five course maps and the readable objectives, topics, summaries, and material links on their Canvas unit pages.
- Produces: Searchable original-language study notes for 20 units, links to all 16 official unit PDFs, and an inventory of 34 unique course, support, institutional, and submitted-work PDFs with reading limits stated.

- [x] **Step 1: Confirm the course-unit inventory**

Review all five courses' module items, including the four unit pages for each. Record which units have a PDF link and which do not; separately classify support documents and submitted-work attachments.

- [x] **Step 2: Write concise notes in original wording**

For each unit, summarize its central idea, key concepts, and a review question. Keep a Canvas link and the official PDF link when available. Do not store or reproduce complete PDF text.

- [x] **Step 3: Update the index and weekly workflow**

Point the README, material inventory, and planning checklist to `NOTAS_DE_ESTUDO_2026.1.md`; state that UVA course PDFs are not kept in the repository, and record any temporary extraction outside Git.

- [x] **Step 4: Verify the text-only inventory**

Confirm that the notes cover 20 units, all 16 official unit PDF links and the additional 18 unique PDFs are categorized in the inventory, no course PDF is tracked or stored in the repository, and `git diff --check` reports no whitespace errors. Record PDF text extraction as partial: U1 and U2 of Development Web were extracted, with summaries only committed.

- [x] **Step 5: Commit the course notes and semester index**

Commit `README.md`, `MATERIAIS_2026.1.md`, `PLANEJAMENTO.md`, and `NOTAS_DE_ESTUDO_2026.1.md` as `docs: add UVA 2026.1 study notes` before the subject archive work.

### Task 4: Audit the complete organization

**Files:**
- Verify: `.gitignore`, `README.md`, `MATERIAIS_2026.1.md`, `materias/`, `originais/`, and `materiais/2026.1/`

**Interfaces:**
- Consumes: Deliverables from Tasks 1–3.
- Produces: Evidence that the requested text is in Git and original binaries remain available locally outside Git.

- [x] **Step 1: Audit tracked text and diagrams**

Review `git ls-files` and confirm it contains the subject Markdown, `.alg`, three UML PNGs, course indexes, and planning docs; it must not list PDF, DOCX, Pages, or original TXT files.

- [x] **Step 2: Audit local originals and course-material links**

Confirm personal-submission originals remain under `originais/`, no UVA course-material PDF is tracked or stored in the repository, and all available course PDF links remain in the semester inventory.

- [x] **Step 3: Audit personal data and repository cleanliness**

Repeat the name/matrícula search across all tracked subject files, run `git diff --check`, inspect the final `git status --short --ignored`, and record any remaining source that could not be converted.

- [x] **Step 4: Commit the final audit/index updates**

Commit only remaining text/index updates. Confirm a clean tracked working tree and verify the final commit log contains no original binaries.
