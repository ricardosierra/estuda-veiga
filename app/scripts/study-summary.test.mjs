import assert from "node:assert/strict";
import test from "node:test";
import {
  pauseSummary,
  resumeSummary,
  speakSummary,
  stopSummary,
} from "../src/ui/summarySpeech.ts";

function installSpeechMock() {
  const calls = { spoken: [], cancelled: 0, paused: 0, resumed: 0 };
  class MockUtterance {
    constructor(text) {
      this.text = text;
      this.lang = "";
      this.onend = null;
      this.onerror = null;
    }
  }
  Object.assign(globalThis, {
    SpeechSynthesisUtterance: MockUtterance,
    speechSynthesis: {
      speak: (utterance) => calls.spoken.push(utterance),
      cancel: () => calls.cancelled++,
      pause: () => calls.paused++,
      resume: () => calls.resumed++,
    },
  });
  return calls;
}

function removeSpeechMock() {
  delete globalThis.SpeechSynthesisUtterance;
  delete globalThis.speechSynthesis;
}

test("speaks_in_portuguese_and_stops_current_utterance", () => {
  const calls = installSpeechMock();
  try {
    assert.equal(speakSummary("conceito"), true);
    assert.equal(calls.spoken.length, 1);
    assert.equal(calls.spoken[0].text, "conceito");
    assert.equal(calls.spoken[0].lang, "pt-BR");

    stopSummary();
    assert.equal(calls.cancelled, 2);
  } finally {
    removeSpeechMock();
  }
});

test("pause_and_resume_delegate_to_speech_synthesis", () => {
  const calls = installSpeechMock();
  try {
    pauseSummary();
    resumeSummary();
    assert.equal(calls.paused, 1);
    assert.equal(calls.resumed, 1);
  } finally {
    removeSpeechMock();
  }
});

test("unsupported_speech_synthesis_returns_unavailable_without_throwing", () => {
  removeSpeechMock();
  assert.equal(speakSummary("sem navegador"), false);
  assert.doesNotThrow(pauseSummary);
  assert.doesNotThrow(resumeSummary);
  assert.doesNotThrow(stopSummary);
});
