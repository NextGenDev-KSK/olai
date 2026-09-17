import { CheckCircle2, HelpCircle, Scale } from 'lucide-react';
import type { QaStatus } from '@/lib/bindings';
import { useTranslation } from '@/i18n/useTranslation';
import { cn } from '@/lib/cn';
import type { TranslationKey } from '@/i18n';

const config: Record<
  QaStatus,
  { icon: typeof CheckCircle2; className: string; key: TranslationKey }
> = {
  answered: { icon: CheckCircle2, className: 'text-success', key: 'ask.status.answered' },
  notInDocuments: {
    icon: HelpCircle,
    className: 'text-ink-muted',
    key: 'ask.status.notInDocuments',
  },
  needsLawyer: { icon: Scale, className: 'text-warning', key: 'ask.status.needsLawyer' },
};

/** A Q&A status pill: icon + word + colour together. */
export function StatusPill({ status }: { status: QaStatus }) {
  const { t } = useTranslation();
  const { icon: Icon, className, key } = config[status];
  return (
    <span
      className={cn(
        'fc-border inline-flex items-center gap-1.5 rounded-full border border-border px-2.5 py-1 text-xs font-semibold',
        className,
      )}
    >
      <Icon aria-hidden="true" className="h-3.5 w-3.5" />
      {t(key)}
    </span>
  );
}
