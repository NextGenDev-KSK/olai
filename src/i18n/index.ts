import type { Language } from '@/lib/bindings';
import en from './en.json';
import ta from './ta.json';
import hi from './hi.json';

/** All translation keys are the keys of the English catalog. */
export type TranslationKey = keyof typeof en;

const catalogs: Record<Language, Record<string, string>> = { en, ta, hi };

/** The languages Olai supports, in display order. */
export const LANGUAGES: readonly Language[] = ['en', 'ta', 'hi'] as const;

/**
 * Translate a key for a language, falling back to English then the key itself.
 * `{name}` placeholders are replaced from `params`.
 */
export function translate(
  lang: Language,
  key: TranslationKey,
  params?: Record<string, string | number>,
): string {
  const catalog = catalogs[lang];
  let value: string = catalog[key] ?? en[key];
  if (params) {
    for (const [name, replacement] of Object.entries(params)) {
      value = value.replace(`{${name}}`, String(replacement));
    }
  }
  return value;
}
