import test from "node:test";
import assert from "node:assert/strict";
import {
  buildFinalGradeInput,
  captureCounts,
  formatGrade,
  hasAnyOption,
  parseGradeField,
} from "../src/ui/canvasAcademic.ts";

function grade(overrides = {}) {
  return {
    assignmentId: 1, name: "AVA 1", pointsPossible: 100, dueAt: null, score: null, grade: null,
    workflowState: null, submittedAt: null, gradedAt: null, late: false, missing: false,
    comments: [], commentsChecked: false, sourceUrl: null, ...overrides,
  };
}

test("grade fields accept Brazilian decimals and reject invalid values", () => {
  assert.equal(parseGradeField("A1", "8,5"), 8.5);
  assert.equal(parseGradeField("A1", " 10 "), 10);
  assert.equal(parseGradeField("A1", ""), null);
  assert.throws(() => parseGradeField("A2", "11"), /entre 0 e 10/);
  assert.throws(() => parseGradeField("A2", "abc"), /7,5/);
  assert.throws(() => parseGradeField("A2", "7,555"), /7,5/);
});

test("final grade input only sends the exams of the chosen discipline kind", () => {
  const fields = { a1: "10", a2: "9", a3: "7", a4: "8" };
  assert.deepEqual(buildFinalGradeInput("regular", fields), { kind: "regular", a1: 10, a2: 9, a3: 7, a4: null });
  assert.deepEqual(buildFinalGradeInput("special", fields), { kind: "special", a1: 10, a2: null, a3: null, a4: 8 });
});

test("grades show the Canvas grade, the score or the pending state", () => {
  assert.equal(formatGrade(grade({ grade: "95", score: 95 })), "95 / 100");
  assert.equal(formatGrade(grade({ score: 87.5 })), "87,5 / 100");
  assert.equal(formatGrade(grade({ missing: true })), "Pendente");
  assert.equal(formatGrade(grade({ pointsPossible: null })), "Sem nota");
});

test("capture counts add up every course", () => {
  const course = (grades, forums) => ({
    courseId: "1", title: "X", kind: "current", sourceUrl: "", grades, forums, announcements: [], warnings: [],
  });
  const capture = {
    capturedAt: "", options: { grades: true, forums: true, announcements: false, inbox: false },
    courses: [
      course([grade({ score: 90, comments: [{ author: "T", createdAt: null, text: "ok", depth: 0 }] }), grade()], []),
      course([], [{ entries: [{}, {}] }]),
    ],
    conversations: [], warnings: [], complete: true, jsonPath: null, markdownPath: null,
  };
  const counts = captureCounts(capture);
  assert.equal(counts.grades, 2);
  assert.equal(counts.graded, 1);
  assert.equal(counts.feedback, 1);
  assert.equal(counts.forums, 1);
  assert.equal(counts.forumEntries, 2);
  assert.equal(hasAnyOption(capture.options), true);
  assert.equal(hasAnyOption({ grades: false, forums: false, announcements: false, inbox: false }), false);
});
