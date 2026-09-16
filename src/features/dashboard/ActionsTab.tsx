import { Shield, FolderOpen, Reply, LifeBuoy, CalendarPlus } from 'lucide-react';
import type { Analysis, ActionGroup } from '@/lib/bindings';
import { Button } from '@/components/Button';
import { CitationChip } from '@/components/CitationChip';
import { useTranslation } from '@/i18n/useTranslation';
import type { TranslationKey } from '@/i18n';
import { buildIcs, downloadText } from '@/lib/export';
import { ReplyPanel } from '@/features/reply/ReplyPanel';

const GROUPS: { group: ActionGroup; key: TranslationKey; icon: typeof Shield }[] = [
  { group: 'protect', key: 'actions.group.protect', icon: Shield },
  { group: 'gather', key: 'actions.group.gather', icon: FolderOpen },
  { group: 'respond', key: 'actions.group.respond', icon: Reply },
  { group: 'getHelp', key: 'actions.group.getHelp', icon: LifeBuoy },
];

/** The Actions tab: a grouped, cited checklist plus .ics export and reply. */
export function ActionsTab({ analysis }: { analysis: Analysis }) {
  const { t } = useTranslation();
  const hasDates = analysis.deadlines.some((d) => d.date);

  const onExportIcs = () => {
    downloadText('olai-reminders.ics', buildIcs(analysis.deadlines), 'text/calendar');
  };

  return (
    <div className="space-y-5">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <h2 className="text-lg font-bold">{t('actions.title')}</h2>
        {hasDates && (
          <Button variant="secondary" onClick={onExportIcs}>
            <CalendarPlus aria-hidden="true" className="h-4 w-4" />
            {t('action.exportIcs')}
          </Button>
        )}
      </div>

      {GROUPS.map(({ group, key, icon: Icon }) => {
        const items = analysis.actions.filter((a) => a.group === group);
        if (items.length === 0) return null;
        return (
          <section key={group} aria-labelledby={`group-${group}`}>
            <h3 id={`group-${group}`} className="inline-flex items-center gap-2 font-semibold">
              <Icon aria-hidden="true" className="h-4 w-4" />
              {t(key)}
            </h3>
            <ul className="mt-1 space-y-1">
              {items.map((a, i) => (
                <li key={i} className="text-sm">
                  <span className="mr-1" aria-hidden="true">
                    •
                  </span>
                  {a.text}
                  {a.citations.length > 0 && (
                    <span className="ml-1 inline-flex flex-wrap gap-1 align-middle">
                      {a.citations.map((c) => (
                        <CitationChip key={`${c.spanId}:${c.quote}`} citation={c} />
                      ))}
                    </span>
                  )}
                </li>
              ))}
            </ul>
          </section>
        );
      })}

      <ReplyPanel />
    </div>
  );
}
