import { useEffect, useState } from 'react';
import * as Tabs from '@radix-ui/react-tabs';
import { Trash2, Keyboard, BookOpen } from 'lucide-react';
import { Button } from '@/components/Button';
import { LanguagePicker } from '@/components/LanguagePicker';
import { HelpTierBanner } from '@/components/HelpTierBanner';
import { ShortcutsDialog } from '@/components/ShortcutsDialog';
import { OverviewTab } from './OverviewTab';
import { ConflictsTab } from './ConflictsTab';
import { ActionsTab } from './ActionsTab';
import { MissingInfoTab } from './MissingInfoTab';
import { AskPanel } from '@/features/ask/AskPanel';
import { CompareView } from '@/features/compare/CompareView';
import { useTranslation } from '@/i18n/useTranslation';
import type { TranslationKey } from '@/i18n';
import { useStore } from '@/store/useStore';

const TABS: { value: string; key: TranslationKey }[] = [
  { value: 'overview', key: 'nav.overview' },
  { value: 'conflicts', key: 'nav.conflicts' },
  { value: 'actions', key: 'nav.actions' },
  { value: 'ask', key: 'nav.ask' },
  { value: 'missing', key: 'nav.missing' },
];

/** The main dashboard: help-tier banner + tabbed views, compare overlay,
 * shortcuts dialog, delete-everything, and easy-read toggle. */
export function DashboardScreen() {
  const { t } = useTranslation();
  const analysis = useStore((s) => s.analysis);
  const deleteEverything = useStore((s) => s.deleteEverything);
  const setActiveConflict = useStore((s) => s.setActiveConflict);
  const toggleEasyRead = useStore((s) => s.toggleEasyRead);
  const easyRead = useStore((s) => s.easyRead);
  const [compareOpen, setCompareOpen] = useState(false);
  const [shortcutsOpen, setShortcutsOpen] = useState(false);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      // Ignore the shortcut while the user is typing in a field.
      const target = e.target as HTMLElement | null;
      const typing =
        target?.tagName === 'INPUT' ||
        target?.tagName === 'TEXTAREA' ||
        target?.isContentEditable === true;
      if (e.key === '?' && !typing) {
        e.preventDefault();
        setShortcutsOpen(true);
      }
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, []);

  if (!analysis) return null;

  const openCompare = (index: number) => {
    setActiveConflict(index);
    setCompareOpen(true);
  };

  const onDelete = () => {
    if (window.confirm(t('action.deleteAllConfirm'))) void deleteEverything();
  };

  return (
    <div className="flex min-h-full flex-col">
      <header className="fc-border flex flex-wrap items-center justify-between gap-2 border-b border-border p-3">
        <h1 className="text-xl font-bold">{t('app.name')}</h1>
        <div className="flex flex-wrap items-center gap-2">
          <LanguagePicker />
          <Button variant="ghost" onClick={toggleEasyRead} aria-pressed={easyRead}>
            <BookOpen aria-hidden="true" className="h-4 w-4" />
            {t('action.easyRead')}
          </Button>
          <Button variant="ghost" onClick={() => setShortcutsOpen(true)} aria-label={t('action.shortcuts')}>
            <Keyboard aria-hidden="true" className="h-5 w-5" />
          </Button>
          <Button variant="danger" onClick={onDelete}>
            <Trash2 aria-hidden="true" className="h-4 w-4" />
            {t('action.deleteAll')}
          </Button>
        </div>
      </header>

      <main id="main" tabIndex={-1} className="flex-1 space-y-4 overflow-y-auto p-4">
        <div aria-live="polite">
          <HelpTierBanner decision={analysis.tier} />
        </div>

        <Tabs.Root defaultValue="overview">
          <Tabs.List aria-label={t('nav.sections')} className="flex flex-wrap gap-1 border-b border-border">
            {TABS.map((tab) => (
              <Tabs.Trigger
                key={tab.value}
                value={tab.value}
                className="fc-border min-h-touch rounded-t-md px-3 py-2 text-sm font-medium data-[state=active]:border-b-2 data-[state=active]:border-brand data-[state=active]:text-brand"
              >
                {t(tab.key)}
              </Tabs.Trigger>
            ))}
          </Tabs.List>
          <Tabs.Content value="overview" className="pt-4 focus-visible:outline-none">
            <OverviewTab analysis={analysis} />
          </Tabs.Content>
          <Tabs.Content value="conflicts" className="pt-4 focus-visible:outline-none">
            <ConflictsTab analysis={analysis} onCompare={openCompare} />
          </Tabs.Content>
          <Tabs.Content value="actions" className="pt-4 focus-visible:outline-none">
            <ActionsTab analysis={analysis} />
          </Tabs.Content>
          <Tabs.Content value="ask" className="pt-4 focus-visible:outline-none">
            <AskPanel />
          </Tabs.Content>
          <Tabs.Content value="missing" className="pt-4 focus-visible:outline-none">
            <MissingInfoTab analysis={analysis} />
          </Tabs.Content>
        </Tabs.Root>
      </main>

      <CompareView open={compareOpen} onOpenChange={setCompareOpen} />
      <ShortcutsDialog open={shortcutsOpen} onOpenChange={setShortcutsOpen} />
    </div>
  );
}
