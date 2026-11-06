import test from "node:test";
import assert from "node:assert/strict";
import {
  capturePrivacyCopy,
  captureProgressLabel,
  captureRunActions,
  groupCaptureItemsByCourse,
  validateCanvasCaptureLimits,
} from "../src/ui/canvasCapture.ts";

function run(status) {
  return {
    id: "run-1",
    scope: "all_visible",
    limitsSnapshot: {
      maxCoursePages: 100,
      maxCourses: 200,
      maxItems: 10000,
      maxPdfBytes: 60 * 1024 * 1024,
      maxRedirects: 4,
      itemTimeoutSeconds: 45,
    },
    status,
    counts: {
      coursesTotal: 2,
      coursesDone: 1,
      itemsTotal: 4,
      captured: 1,
      reused: 0,
      ignored: 0,
      failed: 0,
      needsReview: 0,
      pending: 3,
    },
    currentCourseId: null,
    currentCourseTitle: null,
    currentModuleTitle: null,
    currentItemTitle: null,
    warning: null,
    startedAt: "2026-10-15T12:00:00Z",
    updatedAt: "2026-10-15T12:01:00Z",
    finishedAt: null,
  };
}

test("capture_progress_label_distinguishes_partial_auth_and_completed", () => {
  const labels = ["partial", "waiting_for_login", "completed"].map((status) => captureProgressLabel(run(status)));
  assert.equal(new Set(labels).size, 3);
  assert.match(labels[0], /parcial/i);
  assert.match(labels[1], /login/i);
  assert.match(labels[2], /concluída/i);
});

test("capture_item_groups_preserve_course_module_order", () => {
  const item = (position, courseId, courseTitle, moduleTitle, title) => ({
    position,
    referenceKey: `${courseId}-${position}`,
    contentKey: null,
    canvasItemId: null,
    courseId,
    courseTitle,
    moduleTitle,
    title,
    sourceKind: "canvas_page",
    sourceUrl: "https://uva.instructure.com/courses/1",
    status: "pending",
    retryable: false,
    attempts: 0,
    materialId: null,
    message: null,
    updatedAt: "2026-10-15T12:00:00Z",
  });
  const groups = groupCaptureItemsByCourse([
    item(2, "b", "Estatística", "Unidade 2", "Aula 2"),
    item(1, "a", "Cálculo", "Unidade 1", "Aula 1"),
    item(3, "b", "Estatística", "Unidade 3", "Aula 3"),
  ]);
  assert.deepEqual(groups.map((group) => group.courseTitle), ["Cálculo", "Estatística"]);
  assert.deepEqual(groups[1].items.map(({ moduleTitle }) => moduleTitle), ["Unidade 2", "Unidade 3"]);
});

test("capture_status_copy_never_claims_official_attendance", () => {
  const copy = capturePrivacyCopy().toLocaleLowerCase("pt-BR");
  assert.match(copy, /presença/);
  assert.match(copy, /não registra/);
  assert.match(copy, /canvas/);
});

test("capture_limit_validation_matches_the_backend_bounds", () => {
  const valid = run("completed").limitsSnapshot;
  assert.deepEqual(validateCanvasCaptureLimits(valid), []);
  assert.deepEqual(validateCanvasCaptureLimits({ ...valid, maxCourses: 501 }), ["maxCourses deve ficar entre 1 e 500"]);
  assert.deepEqual(validateCanvasCaptureLimits({ ...valid, maxRedirects: 5 }), ["maxRedirects deve ficar entre 0 e 4"]);
});

test("capture_actions_resume_incomplete_runs_and_cancel_only_running_runs", () => {
  assert.deepEqual(captureRunActions(null), { canStart: true, canResume: false, canCancel: false });
  assert.deepEqual(captureRunActions(run("running")), { canStart: false, canResume: false, canCancel: true });
  assert.deepEqual(captureRunActions(run("partial")), { canStart: true, canResume: true, canCancel: false });
  assert.deepEqual(captureRunActions(run("completed")), { canStart: true, canResume: false, canCancel: false });
});
