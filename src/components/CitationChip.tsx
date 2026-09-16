import { Quote } from 'lucide-react';
import type { Citation } from '@/lib/bindings';
import { useTranslation } from '@/i18n/useTranslation';
import { useStore } from '@/store/useStore';
import { cn } from '@/lib/cn';

/** A clickable citation chip that opens the citation drawer. Unconfirmed
 * citations are greyed and explicitly labelled (never shown as fact). */
export function CitationChip({ citation }: { citation: Citation }) {
  const { t } = useTranslation();
  const openCitation = useStore((s) => s.openCitation);
  return (
    <button
      type="button"
      onClick={() => openCitation(citation)}
      aria-label={`${t('citation.open')} ${citation.spanId}`}
      className={cn(
        'fc-border inline-flex min-h-touch items-center gap-1 rounded border border-border px-2 py-0.5 text-xs',
        citation.confirmed ? 'text-ink' : 'text-ink-muted opacity-70',
      )}
    >
      <Quote aria-hidden="true" className="h-3 w-3" />
      <span>{citation.spanId}</span>
      {!citation.confirmed && <span>· {t('status.notConfirmed')}</span>}
    </button>
  );
}
