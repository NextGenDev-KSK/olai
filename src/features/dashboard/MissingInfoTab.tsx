import { HelpCircle } from 'lucide-react';
import type { Analysis } from '@/lib/bindings';
import { useTranslation } from '@/i18n/useTranslation';

/** The Missing Info tab: what Olai still needs (e.g. a received date). */
export function MissingInfoTab({ analysis }: { analysis: Analysis }) {
  const { t } = useTranslation();
  if (analysis.missingInfo.length === 0) {
    return <p className="text-ink-muted">{t('missing.none')}</p>;
  }
  return (
    <section aria-labelledby="missing-heading">
      <h2 id="missing-heading" className="inline-flex items-center gap-2 text-lg font-bold">
        <HelpCircle aria-hidden="true" className="h-5 w-5" />
        {t('missing.title')}
      </h2>
      <ul className="mt-2 list-inside list-disc text-sm">
        {analysis.missingInfo.map((m, i) => (
          <li key={i}>{m}</li>
        ))}
      </ul>
    </section>
  );
}
