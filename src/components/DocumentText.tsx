import { AlertTriangle } from 'lucide-react';
import type { Document } from '@/lib/bindings';
import { maskPii } from '@/lib/pii';
import { useTranslation } from '@/i18n/useTranslation';
import { cn } from '@/lib/cn';

interface Props {
  doc: Document;
  highlightSpanIds?: ReadonlySet<string>;
  activeSpanId?: string | null;
}

/**
 * Renders a document's spans as escaped text (never dangerouslySetInnerHTML),
 * with PII masked for display and cited spans highlighted. Flagged spans carry
 * a visible warning.
 */
export function DocumentText({ doc, highlightSpanIds, activeSpanId }: Props) {
  const { t } = useTranslation();
  return (
    <div className="space-y-1">
      {doc.spans.map((span) => {
        const highlighted = highlightSpanIds?.has(span.id) ?? false;
        const flagged = span.flags.length > 0;
        const text = maskPii(span.text);
        return (
          <p key={span.id} id={`span-${span.id}`} className="text-sm leading-relaxed">
            {highlighted ? (
              <mark className="cited" data-active={span.id === activeSpanId}>
                {text}
              </mark>
            ) : (
              <span className={cn(flagged && 'text-ink-muted')}>{text}</span>
            )}
            {flagged && (
              <span className="ml-1 inline-flex items-center gap-1 text-xs font-semibold text-warning">
                <AlertTriangle aria-hidden="true" className="h-3 w-3" />
                {t('findings.title')}
              </span>
            )}
          </p>
        );
      })}
    </div>
  );
}
