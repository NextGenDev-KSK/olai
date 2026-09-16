import { useCallback } from 'react';
import { useStore } from '@/store/useStore';
import { translate, type TranslationKey } from './index';

/** Access the translator bound to the current UI language. */
export function useTranslation() {
  const language = useStore((s) => s.language);
  const t = useCallback(
    (key: TranslationKey, params?: Record<string, string | number>) =>
      translate(language, key, params),
    [language],
  );
  return { t, language };
}
