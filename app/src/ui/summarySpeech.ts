type SpeechApi = {
  synthesis: SpeechSynthesis;
  Utterance: typeof SpeechSynthesisUtterance;
};

let playbackGeneration = 0;

function speechApi(): SpeechApi | null {
  const globals = globalThis as typeof globalThis & {
    speechSynthesis?: SpeechSynthesis;
    SpeechSynthesisUtterance?: typeof SpeechSynthesisUtterance;
  };
  if (!globals.speechSynthesis || !globals.SpeechSynthesisUtterance) return null;
  return {
    synthesis: globals.speechSynthesis,
    Utterance: globals.SpeechSynthesisUtterance,
  };
}

export function speakSummary(text: string, onDone?: () => void): boolean {
  const api = speechApi();
  if (!api) return false;
  try {
    const generation = ++playbackGeneration;
    api.synthesis.cancel();
    const utterance = new api.Utterance(text);
    utterance.lang = "pt-BR";
    if (onDone) {
      const finish = () => {
        if (playbackGeneration !== generation) return;
        playbackGeneration++;
        onDone();
      };
      utterance.onend = finish;
      utterance.onerror = finish;
    }
    api.synthesis.speak(utterance);
    return true;
  } catch {
    return false;
  }
}

export function pauseSummary(): void {
  try {
    speechApi()?.synthesis.pause();
  } catch {
    // The system voice may disappear while the WebView is running.
  }
}

export function resumeSummary(): void {
  try {
    speechApi()?.synthesis.resume();
  } catch {
    // The system voice may disappear while the WebView is running.
  }
}

export function stopSummary(): void {
  playbackGeneration++;
  try {
    speechApi()?.synthesis.cancel();
  } catch {
    // Stopping speech must not interfere with the lesson view.
  }
}
