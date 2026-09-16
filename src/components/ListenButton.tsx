import { useEffect, useState } from 'react';
import { Volume2, Square } from 'lucide-react';
import { Button } from './Button';
import { useTranslation } from '@/i18n/useTranslation';
import { speak, stopSpeaking, ttsAvailable } from '@/lib/tts';

/** A "Listen" toggle that reads `text` aloud in the current language. */
export function ListenButton({ text, className }: { text: string; className?: string }) {
  const { t, language } = useTranslation();
  const [speaking, setSpeaking] = useState(false);

  useEffect(() => () => stopSpeaking(), []);

  if (!ttsAvailable()) return null; // graceful fallback: no voice, no button

  const toggle = () => {
    if (speaking) {
      stopSpeaking();
      setSpeaking(false);
    } else {
      setSpeaking(true);
      speak(text, language, () => setSpeaking(false));
    }
  };

  return (
    <Button variant="ghost" onClick={toggle} aria-pressed={speaking} className={className}>
      {speaking ? <Square aria-hidden="true" className="h-4 w-4" /> : <Volume2 aria-hidden="true" className="h-4 w-4" />}
      {speaking ? t('action.stopListening') : t('action.listen')}
    </Button>
  );
}
