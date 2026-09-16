import type { Claim } from '@/lib/bindings';
import { useTranslation } from '@/i18n/useTranslation';
import { cn } from '@/lib/cn';
import { CitationChip } from './CitationChip';

/**
 * Renders a claim's plain text with its citation chips. Unverified claims are
 * greyed and explicitly labelled "Not confirmed", never shown as plain fact.
 */
export function ClaimText({ claim, className }: { claim: Claim; className?: string }) {
  const { t } = useTranslation();
  const notConfirmed = claim.status === 'notConfirmed';
  return (
    <span className={cn(notConfirmed && 'opacity-60', className)}>
      <span>{claim.text}</span>
      {notConfirmed && (
        <span className="ml-1 text-xs italic text-ink-muted">({t('status.notConfirmed')})</span>
      )}
      {claim.citations.length > 0 && (
        <span className="ml-1 inline-flex flex-wrap gap-1 align-middle">
          {claim.citations.map((c) => (
            <CitationChip key={`${c.spanId}:${c.quote}`} citation={c} />
          ))}
        </span>
      )}
    </span>
  );
}
