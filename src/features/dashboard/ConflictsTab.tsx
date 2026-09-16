import type { Analysis } from '@/lib/bindings';
import { Button } from '@/components/Button';
import { ClaimText } from '@/components/ClaimText';
import { SeverityBadge } from '@/components/SeverityBadge';
import { useTranslation } from '@/i18n/useTranslation';

interface Props {
  analysis: Analysis;
  onCompare: (conflictIndex: number) => void;
}

/** The Conflicts tab: numbered notice-vs-agreement discrepancies. */
export function ConflictsTab({ analysis, onCompare }: Props) {
  const { t } = useTranslation();
  if (analysis.conflicts.length === 0) {
    return <p className="text-ink-muted">{t('conflicts.none')}</p>;
  }
  return (
    <div className="space-y-4">
      <p className="font-medium">{t('conflicts.count', { n: analysis.conflicts.length })}</p>
      <ol className="space-y-4">
        {analysis.conflicts.map((c, index) => (
          <li key={c.number} className="fc-border rounded-lg border border-border p-4">
            <div className="mb-2 flex flex-wrap items-center justify-between gap-2">
              <h3 className="font-semibold">
                {c.number}. {c.title}
              </h3>
              <SeverityBadge severity={c.severity} />
            </div>
            <p className="text-sm">
              <span className="font-semibold">{t('conflicts.notice')}:</span>{' '}
              <ClaimText claim={c.noticeSide} />
            </p>
            <p className="mt-1 text-sm">
              <span className="font-semibold">{t('conflicts.agreement')}:</span>{' '}
              <ClaimText claim={c.agreementSide} />
            </p>
            <p className="mt-1 text-sm text-ink-muted">
              <span className="font-semibold">{t('conflicts.explanation')}:</span> {c.explanation}
            </p>
            <div className="mt-3">
              <Button variant="secondary" onClick={() => onCompare(index)}>
                {t('conflicts.viewInCompare')}
              </Button>
            </div>
          </li>
        ))}
      </ol>
    </div>
  );
}
