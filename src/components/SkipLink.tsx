import { useTranslation } from '@/i18n/useTranslation';

/** A keyboard skip link to the main content region. */
export function SkipLink() {
  const { t } = useTranslation();
  return (
    <a href="#main" className="skip-link">
      {t('app.skipToContent')}
    </a>
  );
}
