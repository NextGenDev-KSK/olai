/*
 * jsdom does not implement matchMedia, speechSynthesis, or createObjectURL,
 * even though the DOM lib types declare them as always present. The guards
 * below are genuine runtime checks, so the "unnecessary condition" rule is
 * disabled for this test-support file only.
 */
/* eslint-disable @typescript-eslint/no-unnecessary-condition */
import '@testing-library/jest-dom/vitest';
import { afterEach, expect, vi } from 'vitest';
import { cleanup } from '@testing-library/react';
import * as matchers from 'vitest-axe/matchers';

// Register jest-axe style matchers (toHaveNoViolations).
expect.extend(matchers);

// jsdom lacks these; components rely on them defensively.
if (!window.matchMedia) {
  window.matchMedia = vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  }));
}

// Web Speech API stub so TTS tests do not explode in jsdom.
if (!('speechSynthesis' in window)) {
  Object.defineProperty(window, 'speechSynthesis', {
    writable: true,
    value: {
      speak: vi.fn(),
      cancel: vi.fn(),
      getVoices: vi.fn().mockReturnValue([]),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
    },
  });
}

if (!('SpeechSynthesisUtterance' in globalThis)) {
  class StubUtterance {
    text: string;
    lang = '';
    voice: SpeechSynthesisVoice | null = null;
    onend: (() => void) | null = null;
    onerror: (() => void) | null = null;
    constructor(text: string) {
      this.text = text;
    }
  }
  globalThis.SpeechSynthesisUtterance = StubUtterance as unknown as typeof SpeechSynthesisUtterance;
}

// URL.createObjectURL is used by .ics / .txt export; jsdom has no impl.
if (!URL.createObjectURL) {
  URL.createObjectURL = vi.fn().mockReturnValue('blob:mock');
  URL.revokeObjectURL = vi.fn();
}

// jsdom does not implement scrollIntoView (used by the compare view).
Element.prototype.scrollIntoView = vi.fn();

afterEach(() => {
  cleanup();
  vi.clearAllMocks();
});
