import assert from "node:assert/strict";
import test from "node:test";
import { buildTutorFeedback, getOrBeginLessonSession, submitTutorAnswer } from "../src/ui/studyTypes.ts";

const activity = {
  id: "session-1:0",
  sessionId: "session-1",
  sectionIndex: 0,
  kind: "verifiable",
  prompt: "O que o material chama de entropia?",
  referenceAnswer: "Medida da incerteza.",
  sourcePage: 2,
  sourceHeading: "Página 2",
  sourceExcerpt: "Entropia: medida da incerteza.",
  sourceSha256: "a".repeat(64),
};

test("does not reveal the reference answer before a saved attempt", () => {
  assert.equal(buildTutorFeedback(activity, null, false), null);
});

test("shows an exact reference match as correct without inventing a score", () => {
  assert.deepEqual(buildTutorFeedback(activity, { status: "correct", correct: true }, true), {
    status: "correct",
    tone: "success",
    label: "Resposta correspondente à referência do material.",
    referenceAnswer: "Medida da incerteza.",
    showReference: true,
  });
});

test("sends a paraphrase to source review instead of calling it incorrect", () => {
  assert.deepEqual(buildTutorFeedback(activity, { status: "self_assessed" }, true), {
    status: "self_assessed",
    tone: "review",
    label: "Compare sua resposta com o trecho de referência.",
    referenceAnswer: "Medida da incerteza.",
    showReference: true,
  });
});

test("open questions ask for self-assessment without fabricating a reference answer", () => {
  const open = { ...activity, kind: "self_assessment", referenceAnswer: null };
  assert.deepEqual(buildTutorFeedback(open, { status: "self_assessed" }, true), {
    status: "self_assessed",
    tone: "review",
    label: "Releia o trecho e autoavalie sua explicação.",
    showReference: false,
  });
});

test("history opening reuses the exact saved session without starting a new one", async () => {
  const session = { id: "session-completed", completedAt: "2026-10-14T15:00:00Z" };
  let beginCalls = 0;
  let getCalls = 0;
  const reopened = await getOrBeginLessonSession("session-completed", async () => {
    beginCalls += 1;
    return { id: "wrong-new-session" };
  }, async (id) => {
    getCalls += 1;
    assert.equal(id, "session-completed");
    return session;
  });

  assert.equal(reopened, session);
  assert.equal(beginCalls, 0);
  assert.equal(getCalls, 1);
});

test("scores locally before save and surfaces storage failure without losing the answer", async () => {
  const events = [];
  const response = "A incerteza dos resultados.";
  await assert.rejects(submitTutorAnswer(activity, response, {
    assess: async (submittedActivity, submittedResponse) => {
      events.push(["assess", submittedActivity.id, submittedResponse]);
      return { status: "self_assessed" };
    },
    save: async (_sessionId, _activity, submittedResponse) => {
      events.push(["save", submittedResponse]);
      throw new Error("disco indisponível");
    },
  }), /disco indisponível/);

  assert.deepEqual(events, [
    ["assess", "session-1:0", response],
    ["save", response],
  ]);
});
