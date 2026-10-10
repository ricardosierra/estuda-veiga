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

test("presents heading-free text as one lesson section and splits Markdown headings", () => {
  assert.deepEqual(splitLessonSections("texto simples\n\nsegunda linha"), [
    { title: "Aula", markdown: "texto simples\n\nsegunda linha" },
  ]);
  assert.deepEqual(splitLessonSections("# Um\ntexto\n## Dois\nmais texto").map(({ title }) => title), ["Um", "Dois"]);
});

test("splits extracted PDF page breaks into separate study sections", () => {
  assert.deepEqual(splitLessonSections("texto da página 1\ftexto da página 2"), [
    { title: "Página 1", markdown: "texto da página 1" },
    { title: "Página 2", markdown: "texto da página 2" },
  ]);
});
