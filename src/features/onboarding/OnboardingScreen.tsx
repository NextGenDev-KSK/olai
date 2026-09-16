import { useState } from 'react';
import { ShieldCheck, KeyRound, PlayCircle } from 'lucide-react';
import { Button } from '@/components/Button';
import { LanguagePicker } from '@/components/LanguagePicker';
import { useTranslation } from '@/i18n/useTranslation';
import { useStore } from '@/store/useStore';

/** First-run screen: language, privacy explanation, optional API key, and the
 * "Try demo" entry points. */
export function OnboardingScreen() {
  const { t } = useTranslation();
  const [key, setKey] = useState('');
  const [saved, setSaved] = useState(false);
  const saveApiKey = useStore((s) => s.saveApiKey);
  const loadDemo = useStore((s) => s.loadDemo);
  const loading = useStore((s) => s.loading);

  const onSave = async () => {
    await saveApiKey(key);
    setSaved(true);
    setKey('');
  };

  return (
    <main id="main" tabIndex={-1} className="mx-auto max-w-2xl space-y-8 p-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">{t('onboarding.title')}</h1>
        <LanguagePicker />
      </div>

      <p className="text-ink-muted">{t('onboarding.intro')}</p>

      <section aria-labelledby="privacy-heading" className="fc-border rounded-lg border border-border bg-surface-raised p-4">
        <h2 id="privacy-heading" className="inline-flex items-center gap-2 font-semibold">
          <ShieldCheck aria-hidden="true" className="h-5 w-5 text-success" />
          {t('onboarding.privacyTitle')}
        </h2>
        <p className="mt-2 text-sm text-ink-muted">{t('onboarding.privacy')}</p>
      </section>

      <section aria-labelledby="key-heading" className="space-y-2">
        <h2 id="key-heading" className="inline-flex items-center gap-2 font-semibold">
          <KeyRound aria-hidden="true" className="h-5 w-5" />
          {t('onboarding.apiKeyTitle')}
        </h2>
        <label htmlFor="api-key" className="block text-sm font-medium">
          {t('onboarding.apiKeyLabel')}
        </label>
        <div className="flex flex-wrap gap-2">
          <input
            id="api-key"
            type="password"
            value={key}
            onChange={(e) => setKey(e.target.value)}
            autoComplete="off"
            className="fc-border min-h-touch flex-1 rounded-md border border-border bg-surface px-3 py-2"
            aria-describedby="api-key-help"
          />
          <Button onClick={() => void onSave()} disabled={loading || key.trim() === ''}>
            {t('onboarding.saveKey')}
          </Button>
        </div>
        <p id="api-key-help" className="text-sm text-ink-muted">
          {t('onboarding.apiKeyHelp')}
        </p>
        {saved && (
          <p role="status" className="text-sm font-medium text-success">
            {t('onboarding.keySaved')}
          </p>
        )}
      </section>

      <section aria-labelledby="demo-heading" className="space-y-3">
        <h2 id="demo-heading" className="font-semibold">
          {t('onboarding.orTryDemo')}
        </h2>
        <div className="flex flex-wrap gap-3">
          <Button onClick={() => void loadDemo('eviction')} disabled={loading}>
            <PlayCircle aria-hidden="true" className="h-4 w-4" />
            {t('action.tryDemoEviction')}
          </Button>
          <Button variant="secondary" onClick={() => void loadDemo('scam')} disabled={loading}>
            <PlayCircle aria-hidden="true" className="h-4 w-4" />
            {t('action.tryDemoScam')}
          </Button>
        </div>
      </section>
    </main>
  );
}
