import { AlarmClock, CalendarClock } from 'lucide-react';
import type { Deadline } from '@/lib/bindings';
import { ClaimText } from '@/components/ClaimText';
import { useTranslation } from '@/i18n/useTranslation';
import { cn } from '@/lib/cn';

/** A deadline with its countdown and a transparent "how it was worked out"
 * explanation. Urgency is shown with an icon + words, never colour alone. */
export function DeadlineCountdown({ deadline }: { deadline: Deadline }) {
  const { t } = useTranslation();
  const days = deadline.daysRemaining;
  const urgent = days !== null && days <= 7;

  let status = '';
  if (deadline.calc.needsAnchor) status = t('deadline.needAnchor');
  else if (days === null) status = '';
  else if (days > 0) status = t('deadline.daysLeft', { days });
  else if (days === 0) status = t('deadline.dueToday');
  else status = t('deadline.overdue', { days: Math.abs(days) });

  return (
    <section
      aria-labelledby={`deadline-${deadline.label}`}
      className={cn(
        'fc-border rounded-lg border p-4',
        urgent ? 'border-danger bg-danger/10' : 'border-border bg-surface-raised',
      )}
    >
      <h3
        id={`deadline-${deadline.label}`}
        className="inline-flex items-center gap-2 font-semibold"
      >
        <AlarmClock
          aria-hidden="true"
          className={cn('h-4 w-4', urgent ? 'text-danger' : 'text-info')}
        />
        {t('deadline.title')}: {deadline.label}
      </h3>
      <p className="mt-1 text-2xl font-bold">
        {deadline.date ?? '—'}
        {status && <span className="ml-2 align-middle text-base font-semibold">· {status}</span>}
      </p>
      <p className="mt-1 text-sm">
        <ClaimText claim={deadline.source} />
      </p>
      {deadline.calc.steps.length > 0 && (
        <details className="mt-2 text-sm">
          <summary className="inline-flex cursor-pointer items-center gap-1 font-medium">
            <CalendarClock aria-hidden="true" className="h-4 w-4" />
            {t('deadline.howCalculated')}
          </summary>
          <ul className="mt-1 list-inside list-disc text-ink-muted">
            {deadline.calc.steps.map((s, i) => (
              <li key={i}>{s}</li>
            ))}
          </ul>
        </details>
      )}
    </section>
  );
}
