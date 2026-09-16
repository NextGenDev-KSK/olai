/**
 * Text-to-speech via the WebView2 Web Speech API, with a graceful fallback when
 * no speech synthesis or matching voice is available.
 */
import type { Language } from '@/lib/bindings';

const BCP47: Record<Language, string> = { en: 'en-US', ta: 'ta-IN', hi: 'hi-IN' };

/** Whether speech synthesis is available in this environment. */
export function ttsAvailable(): boolean {
  return typeof window !== 'undefined' && 'speechSynthesis' in window;
}

function pickVoice(lang: Language): SpeechSynthesisVoice | undefined {
  const voices = window.speechSynthesis.getVoices();
  const tag = BCP47[lang];
  return voices.find((v) => v.lang === tag) ?? voices.find((v) => v.lang.startsWith(lang));
}

/** Speak `text` in the given language. No-op if TTS is unavailable. */
export function speak(text: string, lang: Language, onEnd?: () => void): void {
  if (!ttsAvailable() || !text.trim()) {
    onEnd?.();
    return;
  }
  window.speechSynthesis.cancel();
  const utterance = new SpeechSynthesisUtterance(text);
  utterance.lang = BCP47[lang];
  const voice = pickVoice(lang);
  if (voice) utterance.voice = voice;
  if (onEnd) {
    utterance.onend = () => onEnd();
    utterance.onerror = () => onEnd();
  }
  window.speechSynthesis.speak(utterance);
}

/** Stop any ongoing speech. */
export function stopSpeaking(): void {
  if (ttsAvailable()) window.speechSynthesis.cancel();
}
