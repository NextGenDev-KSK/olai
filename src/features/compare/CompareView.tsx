import { useCallback, useEffect, useMemo } from 'react';
import * as Dialog from '@radix-ui/react-dialog';
import { X, ChevronLeft, ChevronRight } from 'lucide-react';
import { Button } from '@/components/Button';
import { DocumentText } from '@/components/DocumentText';
import { SeverityBadge } from '@/components/SeverityBadge';
import { useTranslation } from '@/i18n/useTranslation';
import { useStore } from '@/store/useStore';

/** Two synchronized document panes with linked highlights and J/K navigation
 * between numbered conflicts. */
export function CompareView({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (o: boolean) => void;
}) {
  const { t } = useTranslation();
  const analysis = useStore((s) => s.analysis);
  const docs = useStore((s) => s.docs);
  const active = useStore((s) => s.activeConflict);
  const setActive = useStore((s) => s.setActiveConflict);

  const conflicts = analysis?.conflicts ?? [];
  const total = conflicts.length;
  const conflict = conflicts[active];

  const noticeDoc = docs.find((d) => d.role === 'notice');
  const agreementDoc = docs.find((d) => d.role === 'myAgreement');

  const noticeSpans = useMemo(
    () => new Set(conflict?.noticeSide.citations.map((c) => c.spanId) ?? []),
    [conflict],
  );
  const agreementSpans = useMemo(
    () => new Set(conflict?.agreementSide.citations.map((c) => c.spanId) ?? []),
    [conflict],
  );

  const go = useCallback(
    (delta: number) => {
      if (total > 0) setActive((active + delta + total) % total);
    },
    [active, total, setActive],
  );

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'j' || e.key === 'J') {
        e.preventDefault();
        go(1);
      } else if (e.key === 'k' || e.key === 'K') {
        e.preventDefault();
        go(-1);
      }
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [open, go]);

  useEffect(() => {
    if (!open || !conflict) return;
    for (const id of [
      conflict.noticeSide.citations[0]?.spanId,
      conflict.agreementSide.citations[0]?.spanId,
    ]) {
      if (id) document.getElementById(`span-${id}`)?.scrollIntoView({ block: 'center' });
    }
  }, [open, active, conflict]);

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50" />
        <Dialog.Content
          className="fc-border fixed inset-2 z-50 flex flex-col rounded-lg border border-border bg-surface p-4"
          aria-describedby={undefined}
        >
          <div className="flex items-center justify-between">
            <Dialog.Title className="text-lg font-bold">{t('compare.title')}</Dialog.Title>
            <Dialog.Close asChild>
              <Button variant="ghost" aria-label={t('action.close')}>
                <X aria-hidden="true" className="h-5 w-5" />
              </Button>
            </Dialog.Close>
          </div>

          {conflict && (
            <div className="my-2">
              <div className="flex flex-wrap items-center gap-3">
                <Button
                  variant="secondary"
                  onClick={() => go(-1)}
                  aria-label={t('shortcuts.conflicts')}
                >
                  <ChevronLeft aria-hidden="true" className="h-4 w-4" />
                </Button>
                <p aria-live="polite" className="flex flex-wrap items-center gap-2 font-medium">
                  <span>
                    {t('compare.conflictOf', { n: active + 1, total })}: {conflict.title}
                  </span>
                  <SeverityBadge severity={conflict.severity} />
                </p>
                <Button
                  variant="secondary"
                  onClick={() => go(1)}
                  aria-label={t('shortcuts.conflicts')}
                >
                  <ChevronRight aria-hidden="true" className="h-4 w-4" />
                </Button>
              </div>
              <p className="mt-1 text-sm text-ink-muted">{t('compare.navHint')}</p>
            </div>
          )}

          <div className="mt-2 grid flex-1 grid-cols-1 gap-3 overflow-hidden md:grid-cols-2">
            <section
              aria-label={t('compare.notice')}
              className="fc-border overflow-y-auto rounded-md border border-border p-3"
            >
              <h3 className="mb-2 font-semibold">{t('compare.notice')}</h3>
              {noticeDoc && (
                <DocumentText
                  doc={noticeDoc}
                  highlightSpanIds={noticeSpans}
                  activeSpanId={conflict?.noticeSide.citations[0]?.spanId ?? null}
                />
              )}
            </section>
            <section
              aria-label={t('compare.agreement')}
              className="fc-border overflow-y-auto rounded-md border border-border p-3"
            >
              <h3 className="mb-2 font-semibold">{t('compare.agreement')}</h3>
              {agreementDoc && (
                <DocumentText
                  doc={agreementDoc}
                  highlightSpanIds={agreementSpans}
                  activeSpanId={conflict?.agreementSide.citations[0]?.spanId ?? null}
                />
              )}
            </section>
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
