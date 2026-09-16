import { describe, it, expect, vi, beforeEach } from 'vitest';
import { speak, stopSpeaking, ttsAvailable } from './tts';

describe('tts', () => {
  beforeEach(() => vi.clearAllMocks());

  it('reports availability in jsdom (stubbed)', () => {
    expect(ttsAvailable()).toBe(true);
  });

  it('speaks non-empty text', () => {
    speak('hello', 'en');
    expect(window.speechSynthesis.speak).toHaveBeenCalledOnce();
  });

  it('invokes the onEnd callback immediately for empty text', () => {
    const onEnd = vi.fn();
    speak('   ', 'en', onEnd);
    expect(onEnd).toHaveBeenCalledOnce();
    expect(window.speechSynthesis.speak).not.toHaveBeenCalled();
  });

  it('cancels speech on stop', () => {
    stopSpeaking();
    expect(window.speechSynthesis.cancel).toHaveBeenCalled();
  });
});
