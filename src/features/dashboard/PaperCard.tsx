import { FileText } from 'lucide-react';
import type { PaperCard as PaperCardData } from '@/lib/bindings';
import { ClaimText } from '@/components/ClaimText';
import { ListenButton } from '@/components/ListenButton';
import { useTranslation } from '@/i18n/useTranslation';

/** The "Paper Card": a plain-language summary of what the document is. */
export function PaperCard({ card }: { card: PaperCardData }) {
  const { t } = useTranslation();
  return (
    <section
      aria-labelledby="paper-heading"
      className="fc-border rounded-lg border border-border bg-surface-raised p-4"
    >
      <div className="mb-2 flex items-center justify-between">
        <h2 id="paper-heading" className="inline-flex items-center gap-2 text-lg font-bold">
          <FileText aria-hidden="true" className="h-5 w-5" />
          {t('paper.title')}
        </h2>
        <ListenButton text={card.summary.text} />
      </div>
      <dl className="space-y-3 text-sm">
        <div>
          <dt className="font-semibold">{t('paper.type')}</dt>
          <dd>
            <ClaimText claim={card.docType} />
          </dd>
        </div>
        {card.parties.length > 0 && (
          <div>
            <dt className="font-semibold">{t('paper.parties')}</dt>
            <dd>
              <ul className="list-inside list-disc">
                {card.parties.map((p) => (
                  <li key={p.role}>
                    <span className="font-medium">{p.role}:</span> <ClaimText claim={p.claim} />
                  </li>
                ))}
              </ul>
            </dd>
          </div>
        )}
        <div>
          <dt className="font-semibold">{t('paper.summary')}</dt>
          <dd>
            <ClaimText claim={card.summary} />
          </dd>
        </div>
      </dl>
    </section>
  );
}
