import assert from "node:assert/strict";
import test from "node:test";
import { splitLessonSections, tutorSectionInput } from "../src/ui/studyTypes.ts";

test("prepares a local tutor source excerpt without inventing a page", () => {
  const [section] = splitLessonSections("## Introdução\nConceito: conteúdo local.");

  assert.deepEqual(tutorSectionInput(section), {
    index: 0,
    title: "Introdução",
    markdown: "## Introdução\nConceito: conteúdo local.",
  });
});

test("retains the full section and explicit page for backend revision hashing", () => {
  const markdown = "## Página 3\n" + "x".repeat(700);
  const [section] = splitLessonSections(markdown);
  const input = tutorSectionInput(section);

  assert.equal(input.sourcePage, 3);
  assert.equal(input.markdown, markdown);
  assert.equal(input.markdown.length, markdown.length);
});
