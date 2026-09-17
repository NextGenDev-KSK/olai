import { Info } from 'lucide-react';
import { useTranslation } from '@/i18n/useTranslation';

/** The persistent, always-visible legal disclaimer footer. */
export function DisclaimerFooter() {
  const { t } = useTranslation();
  return (
    <footer className="fc-border sticky bottom-0 z-10 flex items-center gap-2 border-t border-border bg-surface-raised px-4 py-2 text-sm text-ink-muted">
      <Info aria-hidden="true" className="h-4 w-4 shrink-0" />
      <p>{t('app.disclaimer')}</p>
    </footer>
  );
}
