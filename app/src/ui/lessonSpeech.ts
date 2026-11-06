import { useEffect, useRef, useState } from "react";

export type LocalSpeechVoice = {
  name: string;
  lang: string;
  localService: boolean;
  nativeVoice?: SpeechSynthesisVoice;
};

export type SpeechHandlers = {
  onstart: () => void;
  onend: () => void;
  onerror: () => void;
};

export type SpeechPort = {
  getVoices: () => LocalSpeechVoice[];
  speak: (text: string, voice: LocalSpeechVoice, handlers: SpeechHandlers) => void;
  pause: () => void;
  resume: () => void;
  cancel: () => void;
  subscribeVoicesChanged?: (handler: () => void) => () => void;
};

export type LessonSpeechState = {
  available: boolean;
  speaking: boolean;
  paused: boolean;
  voiceLabel: string;
};

export function speechTextFromMarkdown(markdown: string): string {
  return markdown
    .replace(/```[\s\S]*?```|~~~[\s\S]*?~~~/g, " ")
    .replace(/!\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/!?\[([^\]]+)\]\[[^\]]*\]/g, "$1")
    .replace(/^\s*\[[^\]]+\]:\s*\S+.*$/gm, " ")
    .replace(/\[([^\]]+)\]\([^)]*\)/g, "$1")
    .replace(/\$\$[\s\S]*?\$\$[.,;:!?]?|\\\[[\s\S]*?\\\][.,;:!?]?|\\\([\s\S]*?\\\)[.,;:!?]?|\$[^$\n]+\$[.,;:!?]?/g, " ")
    .replace(/`+[^`]*`+/g, " ")
    .replace(/^\s*[-*_]{3,}\s*$/gm, " ")
    .replace(/^\s*\|?\s*:?-{3,}.*$/gm, " ")
    .replace(/^\s*#{1,6}\s*/gm, "")
    .replace(/^\s*>\s?/gm, "")
    .replace(/^\s*(?:[-+*]|\d+[.)])\s+/gm, "")
    .replace(/<[^>]*>/g, " ")
    .replace(/[|]/g, " ")
    .replace(/[*_~]{1,3}/g, "")
    .replace(/\\([\\`*{}\[\]()#+.!_>-])/g, "$1")
    .replace(/[\t\r\n ]+/g, " ")
    .replace(/\s+([,;:!?])/g, "$1")
    .trim();
}

export class LessonSpeechController {
  private text: string;
  private identity: string;
  private voice: LocalSpeechVoice | undefined;
  private active = false;
  private generation = 0;
  private unsubscribeVoices: (() => void) | undefined;
  private readonly port: SpeechPort | null;
  private readonly listeners = new Set<(state: LessonSpeechState) => void>();
  private currentState: LessonSpeechState = {
    available: false,
    speaking: false,
    paused: false,
    voiceLabel: "",
  };

  constructor(port: SpeechPort | null, text = "", identity = text) {
    this.port = port;
    this.text = text;
    this.identity = identity;
    this.refreshVoice();
    this.start();
  }

  get state(): LessonSpeechState {
    return { ...this.currentState };
  }

  subscribe(listener: (state: LessonSpeechState) => void): () => void {
    this.listeners.add(listener);
    listener(this.state);
    return () => this.listeners.delete(listener);
  }

  start(): void {
    if (!this.port || this.unsubscribeVoices) return;
    this.unsubscribeVoices = this.port.subscribeVoicesChanged?.(() => this.refreshVoice());
    this.refreshVoice();
  }

  setText(text: string, identity = text): void {
    if (text === this.text && identity === this.identity) return;
    this.text = text;
    this.identity = identity;
    if (this.active) this.stop();
  }

  speak(onEnd?: () => void): boolean {
    const text = speechTextFromMarkdown(this.text);
    this.refreshVoice();
    if (!this.port || !this.voice || !text) return false;
    if (this.active) this.stop();

    const voice = this.voice;
    const generation = ++this.generation;
    this.active = true;
    this.update({ speaking: false, paused: false });
    try {
      this.port.speak(text, voice, {
        onstart: () => {
          if (generation === this.generation && this.active) this.update({ speaking: true, paused: false });
        },
        onend: () => {
          if (this.finish(generation)) onEnd?.();
        },
        onerror: () => {
          if (generation === this.generation) {
            this.active = false;
            this.update({ available: false, speaking: false, paused: false, voiceLabel: "" });
          }
        },
      });
      return true;
    } catch {
      this.active = false;
      this.update({ available: false, speaking: false, paused: false, voiceLabel: "" });
      return false;
    }
  }

  pause(): void {
    if (!this.port || !this.active || this.currentState.paused) return;
    this.port.pause();
    this.update({ speaking: false, paused: true });
  }

  resume(): void {
    if (!this.port || !this.active || !this.currentState.paused) return;
    this.port.resume();
    this.update({ speaking: true, paused: false });
  }

  stop(): void {
    this.cancelActive(true);
  }

  dispose(): void {
    this.cancelActive(true);
    this.unsubscribeVoices?.();
    this.unsubscribeVoices = undefined;
    this.listeners.clear();
  }

  private refreshVoice(): void {
    const voices = this.port?.getVoices() ?? [];
    this.voice = chooseLocalVoice(voices);
    this.update({ available: !!this.voice, voiceLabel: this.voice?.name ?? "" });
  }

  private finish(generation: number): boolean {
    if (generation !== this.generation || !this.active) return false;
    this.active = false;
    this.update({ speaking: false, paused: false });
    return true;
  }

  private cancelActive(force: boolean): void {
    if (!this.port || (!this.active && !force)) return;
    this.generation += 1;
    this.active = false;
    this.port.cancel();
    this.update({ speaking: false, paused: false });
  }

  private update(partial: Partial<LessonSpeechState>): void {
    this.currentState = { ...this.currentState, ...partial };
    for (const listener of this.listeners) listener(this.state);
  }
}

function chooseLocalVoice(voices: LocalSpeechVoice[]): LocalSpeechVoice | undefined {
  const local = voices.filter((voice) => voice.localService === true);
  return local.find((voice) => /^pt[-_]br$/i.test(voice.lang))
    ?? local.find((voice) => /^pt(?:[-_]|$)/i.test(voice.lang))
    ?? local[0];
}

function browserSpeechPort(): SpeechPort | null {
  if (typeof window === "undefined" || !window.speechSynthesis || typeof SpeechSynthesisUtterance === "undefined") return null;
  const synthesis = window.speechSynthesis;
  return {
    getVoices: () => synthesis.getVoices().map((nativeVoice) => ({
      name: nativeVoice.name,
      lang: nativeVoice.lang,
      localService: nativeVoice.localService === true,
      nativeVoice,
    })),
    speak: (text, voice, handlers) => {
      if (!voice.nativeVoice) throw new Error("A voz local não está disponível.");
      const utterance = new SpeechSynthesisUtterance(text);
      utterance.voice = voice.nativeVoice;
      utterance.lang = voice.lang || "pt-BR";
      utterance.onstart = handlers.onstart;
      utterance.onend = handlers.onend;
      utterance.onerror = handlers.onerror;
      synthesis.speak(utterance);
    },
    pause: () => synthesis.pause(),
    resume: () => synthesis.resume(),
    cancel: () => synthesis.cancel(),
    subscribeVoicesChanged: (handler) => {
      synthesis.addEventListener("voiceschanged", handler);
      return () => synthesis.removeEventListener("voiceschanged", handler);
    },
  };
}

export function useLessonSpeech(text: string, identity = text) {
  const controllerRef = useRef<LessonSpeechController | null>(null);
  if (!controllerRef.current) controllerRef.current = new LessonSpeechController(browserSpeechPort(), text, identity);
  const controller = controllerRef.current;
  const [state, setState] = useState(controller.state);

  useEffect(() => {
    const unsubscribe = controller.subscribe(setState);
    controller.start();
    return () => {
      unsubscribe();
      controller.dispose();
    };
  }, [controller]);

  useEffect(() => {
    controller.setText(text, identity);
  }, [controller, text, identity]);

  return {
    ...state,
    speak: (onEnd?: () => void) => controller.speak(onEnd),
    pause: () => controller.pause(),
    resume: () => controller.resume(),
    stop: () => controller.stop(),
  };
}
