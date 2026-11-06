import assert from "node:assert/strict";
import test from "node:test";
import { documentsForEnrollment, materialsForEnrollment, splitLessonSections } from "../src/ui/studyTypes.ts";

const enrollment = {
  id: "cc-bases-2026.2",
  title: "Bases de Programação",
  semester: "2026.2",
  status: "enrolled",
};

test("associates a document only when course and explicit semester match", () => {
  const documents = [
    { relativePath: "materias/bases/aula.md", title: "Aula", format: "markdown", sizeBytes: 10, course: "Bases de Programacao", semester: "2026.2" },
    { relativePath: "materias/bases/sem-periodo.md", title: "Aula antiga", format: "markdown", sizeBytes: 10, course: "Bases de Programação" },
    { relativePath: "materias/bases/outro-periodo.md", title: "Aula futura", format: "markdown", sizeBytes: 10, course: "Bases de Programação", semester: "2026.1" },
    { relativePath: "materias/outro-curso/aula.md", title: "Aula", format: "markdown", sizeBytes: 10, course: "Paradigmas de Programação", semester: "2026.2" },
  ];

  assert.deepEqual(documentsForEnrollment(enrollment, documents).map((item) => item.relativePath), ["materias/bases/aula.md"]);
});

test("keeps a document unassociated when the enrollment has no semester", () => {
  const documents = [
    { relativePath: "materias/bases/aula.md", title: "Aula", format: "markdown", sizeBytes: 10, course: "Bases de Programação", semester: "2026.2" },
  ];

  assert.deepEqual(documentsForEnrollment({ ...enrollment, semester: undefined }, documents), []);
});

test("associates imported Canvas lessons only to their recorded semester", () => {
  const materials = [
    { id: "current", courseTitle: "Bases de Programação", semester: "2026.2" },
    { id: "old", courseTitle: "Bases de Programacao", semester: "2026.1" },
    { id: "unknown", courseTitle: "Bases de Programação", semester: null },
  ];

  assert.deepEqual(materialsForEnrollment(enrollment, materials).map((item) => item.id), ["current"]);
});

test("shows shared Canvas material through its matching course association", () => {
  const materials = [
    {
      id: "shared-current",
      courseId: "57999",
      courseTitle: "Curso histórico compartilhado",
      semester: null,
      associations: [
        { materialId: "shared-current", courseId: "57827", courseTitle: "Bases de Programação", moduleTitle: "Unidade 1", itemTitle: "Aula", semester: "2026.2" },
      ],
    },
    {
      id: "shared-wrong-term",
      courseId: "57999",
      courseTitle: "Curso histórico compartilhado",
      semester: null,
      associations: [
        { materialId: "shared-wrong-term", courseId: "57000", courseTitle: "Bases de Programação", moduleTitle: "Unidade antiga", itemTitle: "Aula antiga", semester: "2026.1" },
      ],
    },
  ];

  assert.deepEqual(materialsForEnrollment(enrollment, materials).map(({ id }) => id), ["shared-current"]);
});

test("keeps unknown-semester Canvas material out of a confirmed active enrollment", () => {
  const materials = [{
    id: "unknown-term",
    courseTitle: "Curso compartilhado",
    semester: null,
    associations: [{ materialId: "unknown-term", courseId: "57904", courseTitle: "Bases de Programação", moduleTitle: "Unidade 1", itemTitle: "Aula", semester: null }],
  }];

  assert.deepEqual(materialsForEnrollment(enrollment, materials).map(({ id }) => id), []);
});

test("presents heading-free text as one lesson section and splits Markdown headings", () => {
  assert.deepEqual(splitLessonSections("texto simples\n\nsegunda linha"), [
    { index: 0, title: "Aula", markdown: "texto simples\n\nsegunda linha" },
  ]);
  assert.deepEqual(splitLessonSections("# Um\ntexto\n## Dois\nmais texto").map(({ title }) => title), ["Um", "Dois"]);
});

test("splits extracted PDF page breaks into separate study sections", () => {
  assert.deepEqual(splitLessonSections("texto da página 1\ftexto da página 2"), [
    { index: 0, title: "Página 1", markdown: "texto da página 1", sourcePage: 1 },
    { index: 1, title: "Página 2", markdown: "texto da página 2", sourcePage: 2 },
  ]);
});

test("uses explicit page headings as source metadata without changing their Markdown", () => {
  assert.deepEqual(splitLessonSections("## Página 7\nDistribuição normal\n\nTexto de origem."), [
    { index: 0, title: "Página 7", markdown: "## Página 7\nDistribuição normal\n\nTexto de origem.", sourcePage: 7 },
  ]);
});

test("keeps Markdown sections without a page number unnumbered and ordered", () => {
  assert.deepEqual(splitLessonSections("## Introdução\nTexto.\n## Exemplo\nOutro trecho."), [
    { index: 0, title: "Introdução", markdown: "## Introdução\nTexto." },
    { index: 1, title: "Exemplo", markdown: "## Exemplo\nOutro trecho." },
  ]);
});

test("uses explicit PDF page headings as boundaries while keeping headings inside each page", () => {
  const sections = splitLessonSections(
    "# Unidade 1 — Material\n> Disciplina: Estatística\n> Páginas no PDF: 5\n\n## Página 4\nTexto da página quatro.\n\n### Exemplo\nMais conteúdo.\n\n## Página 5\nTexto da página cinco.",
  );

  assert.deepEqual(sections, [
    {
      index: 0,
      title: "Página 4",
      markdown: "## Página 4\nTexto da página quatro.\n\n### Exemplo\nMais conteúdo.",
      sourcePage: 4,
    },
    {
      index: 1,
      title: "Página 5",
      markdown: "## Página 5\nTexto da página cinco.",
      sourcePage: 5,
    },
  ]);
});

test("omits title-only and publication-credit PDF pages without renumbering source pages", () => {
  const sections = splitLessonSections(
    "# E-book de Bases de Programação — Capítulo 1\n> PDF original preservado em originais.\n\n## Página 1\nBases de Programação\n\n## Página 2\n## Página 3\nAlgoritmos organizam uma sequência de instruções para resolver problemas.",
  );

  assert.deepEqual(sections, [
    {
      index: 0,
      title: "Página 3",
      markdown: "## Página 3\nAlgoritmos organizam uma sequência de instruções para resolver problemas.",
      sourcePage: 3,
    },
  ]);
});
