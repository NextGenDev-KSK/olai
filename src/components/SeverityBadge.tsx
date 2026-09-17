import { AlertTriangle, AlertCircle, Info } from 'lucide-react';
import type { Severity } from '@/lib/bindings';
import { useTranslation } from '@/i18n/useTranslation';
import { cn } from '@/lib/cn';

const config: Record<
  Severity,
  {
    icon: typeof Info;
    className: string;
    key: 'severity.low' | 'severity.medium' | 'severity.high';
  }
> = {
  low: { icon: Info, className: 'text-info', key: 'severity.low' },
  medium: { icon: AlertCircle, className: 'text-warning', key: 'severity.medium' },
  high: { icon: AlertTriangle, className: 'text-danger', key: 'severity.high' },
};

/**
 * Severity shown as icon + word + colour together — never colour alone
 * (WCAG 1.4.1). The word is a real text node for screen readers.
 */
export function SeverityBadge({ severity }: { severity: Severity }) {
  const { t } = useTranslation();
  const { icon: Icon, className, key } = config[severity];
  return (
    <span className={cn('inline-flex items-center gap-1.5 font-semibold', className)}>
      <Icon aria-hidden="true" className="h-4 w-4" />
      <span>
        {t('severity.label')}: {t(key)}
      </span>
    </span>
  );
}
