import type { Language } from '@/lib/bindings';
import { useTranslation } from '@/i18n/useTranslation';
import { useStore } from '@/store/useStore';

const OPTIONS: { value: Language; key: 'lang.en' | 'lang.ta' | 'lang.hi' }[] = [
  { value: 'en', key: 'lang.en' },
  { value: 'ta', key: 'lang.ta' },
  { value: 'hi', key: 'lang.hi' },
];

/** A labelled language selector for the UI and content language. */
export function LanguagePicker() {
  const { t, language } = useTranslation();
  const setLanguage = useStore((s) => s.setLanguage);
  return (
    <div className="flex items-center gap-2">
      <label htmlFor="language-select" className="text-sm font-medium">
        {t('lang.label')}
      </label>
      <select
        id="language-select"
        value={language}
        onChange={(e) => void setLanguage(e.target.value as Language)}
        className="fc-border min-h-touch rounded-md border border-border bg-surface px-2 py-1 text-sm"
      >
        {OPTIONS.map((o) => (
          <option key={o.value} value={o.value}>
            {t(o.key)}
          </option>
        ))}
      </select>
    </div>
  );
}
