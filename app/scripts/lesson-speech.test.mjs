import assert from "node:assert/strict";
import test from "node:test";
import { LessonSpeechController, speechTextFromMarkdown } from "../src/ui/lessonSpeech.ts";

function speechMock(voices = []) {
  const calls = { spoken: [], cancelled: 0, paused: 0, resumed: 0, unsubscribe: 0 };
  let voiceChange;
  const port = {
    getVoices: () => voices,
    speak(text, voice, handlers) {
      calls.spoken.push({ text, voice, handlers });
    },
    pause: () => { calls.paused += 1; },
    resume: () => { calls.resumed += 1; },
    cancel: () => { calls.cancelled += 1; },
    subscribeVoicesChanged(handler) {
      voiceChange = handler;
      return () => { calls.unsubscribe += 1; };
    },
    emitVoicesChanged() { voiceChange?.(); },
    calls,
  };
  return port;
}

test("cleans Markdown, link targets, formulas and code while keeping image alt text", () => {
  const text = speechTextFromMarkdown(
    "# Aula\n\nLeia **este conceito** e [o material](https://example.test). ![diagrama da árvore](arvore.png)\n\n`x = 2` e $x^2 + y^2$.\n\n```js\nconsole.log('não narrar')\n```",
  );

  assert.equal(text, "Aula Leia este conceito e o material. diagrama da árvore e");
});

test("chooses the local Brazilian Portuguese voice even when another local voice is listed first", () => {
  const english = { name: "Local English", lang: "en-US", localService: true };
  const portuguese = { name: "Português local", lang: "pt-BR", localService: true };
  const controller = new LessonSpeechController(speechMock([english, portuguese]), "Texto da aula");

  assert.deepEqual(controller.state, {
    available: true,
    speaking: false,
    paused: false,
    voiceLabel: "Português local",
  });
  assert.equal(controller.speak(), true);
  assert.equal(controller.state.speaking, false);
  assert.equal(controller.state.voiceLabel, "Português local");
});

test("never falls back to remote voices and remains usable when speech is unavailable", () => {
  const remote = { name: "Remote Brazilian Portuguese", lang: "pt-BR", localService: false };
  const remotePort = speechMock([remote]);
  const remoteController = new LessonSpeechController(remotePort, "Texto");
  const absentController = new LessonSpeechController(null, "Texto");

  assert.equal(remoteController.state.available, false);
  assert.equal(remoteController.speak(), false);
  assert.equal(remotePort.calls.spoken.length, 0);
  assert.equal(absentController.state.available, false);
  assert.equal(absentController.speak(), false);
});

test("pause, resume, stop, section changes and unmount cancel the active utterance", () => {
  const port = speechMock([{ name: "Voz local", lang: "pt-BR", localService: true }]);
  const controller = new LessonSpeechController(port, "Seção um");

  assert.equal(controller.speak(), true);
  port.calls.spoken[0].handlers.onstart();
  assert.equal(controller.state.speaking, true);
  controller.pause();
  assert.equal(controller.state.paused, true);
  controller.resume();
  assert.equal(controller.state.speaking, true);
  controller.stop();
  assert.equal(controller.state.speaking, false);
  assert.equal(controller.state.paused, false);

  controller.speak();
  controller.setText("Seção dois");
  assert.equal(controller.state.speaking, false);
  assert.equal(port.calls.cancelled, 2);
  assert.equal(port.calls.spoken.length, 2);

  controller.dispose();
  assert.equal(port.calls.unsubscribe, 1);
  assert.equal(port.calls.cancelled, 3);
});

test("voice changes refresh local voice availability without starting playback", () => {
  const port = speechMock([]);
  const controller = new LessonSpeechController(port, "Texto");
  assert.equal(controller.state.available, false);

  port.getVoices = () => [{ name: "Voz local", lang: "pt-BR", localService: true }];
  port.emitVoicesChanged();

  assert.equal(controller.state.available, true);
  assert.equal(port.calls.spoken.length, 0);
});

test("signals natural page narration completion once and never advances after stop", () => {
  const port = speechMock([{ name: "Voz local", lang: "pt-BR", localService: true }]);
  const controller = new LessonSpeechController(port, "Texto da página");
  let advances = 0;

  assert.equal(controller.speak(() => { advances += 1; }), true);
  port.calls.spoken[0].handlers.onend();
  assert.equal(advances, 1);

  assert.equal(controller.speak(() => { advances += 1; }), true);
  const staleHandlers = port.calls.spoken[1].handlers;
  controller.stop();
  staleHandlers.onend();
  assert.equal(advances, 1);
});
