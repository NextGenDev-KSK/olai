import { ShieldAlert } from 'lucide-react';
import type { InjectionFinding } from '@/lib/bindings';
import { useTranslation } from '@/i18n/useTranslation';

/** Surfaces prompt-injection / hidden-text findings detected in the documents. */
export function FindingsPanel({ findings }: { findings: InjectionFinding[] }) {
  const { t } = useTranslation();
  return (
    <section
      aria-labelledby="findings-heading"
      className="fc-border rounded-lg border-2 border-warning bg-warning/10 p-4"
    >
      <h2 id="findings-heading" className="inline-flex items-center gap-2 text-lg font-bold">
        <ShieldAlert aria-hidden="true" className="h-5 w-5 text-warning" />
        {t('findings.title')}
      </h2>
      <p className="mt-1 text-sm">{t('findings.intro')}</p>
      <ul className="mt-2 space-y-1 text-sm">
        {findings.map((f, i) => (
          <li key={`${f.spanId}-${i}`}>
            <span className="font-mono">{f.spanId}</span> — {f.reason}: “{f.excerpt}”
          </li>
        ))}
      </ul>
    </section>
  );
}
