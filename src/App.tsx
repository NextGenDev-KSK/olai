import { useEffect } from 'react';
import { SkipLink } from '@/components/SkipLink';
import { DisclaimerFooter } from '@/components/DisclaimerFooter';
import { CitationDrawer } from '@/components/CitationDrawer';
import { OnboardingScreen } from '@/features/onboarding/OnboardingScreen';
import { DashboardScreen } from '@/features/dashboard/DashboardScreen';
import { useStore } from '@/store/useStore';
import { cn } from '@/lib/cn';

/** Root component: screen switching, language attribute, and global chrome. */
export default function App() {
  const screen = useStore((s) => s.screen);
  const language = useStore((s) => s.language);
  const easyRead = useStore((s) => s.easyRead);
  const init = useStore((s) => s.init);

  useEffect(() => {
    void init();
  }, [init]);

  useEffect(() => {
    document.documentElement.lang = language;
  }, [language]);

  return (
    <div lang={language} className={cn('flex min-h-screen flex-col', easyRead && 'text-lg leading-loose')}>
      <SkipLink />
      <div className="flex-1">
        {screen === 'onboarding' ? <OnboardingScreen /> : <DashboardScreen />}
      </div>
      <CitationDrawer />
      <DisclaimerFooter />
    </div>
  );
}
