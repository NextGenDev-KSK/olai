import * as Dialog from '@radix-ui/react-dialog';
import { CheckCircle2, CircleSlash, X } from 'lucide-react';
import { useTranslation } from '@/i18n/useTranslation';
import { useStore } from '@/store/useStore';
import { Button } from './Button';

/** A drawer showing the exact quote, page, and verification status of the
 * citation the user selected. */
export function CitationDrawer() {
  const { t } = useTranslation();
  const citation = useStore((s) => s.activeCitation);
  const close = useStore((s) => s.closeCitation);
  const open = citation !== null;

  return (
    <Dialog.Root open={open} onOpenChange={(o) => !o && close()}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/40" />
        <Dialog.Content
          className="fc-border fixed right-0 top-0 z-50 h-full w-full max-w-md overflow-y-auto border-l border-border bg-surface p-5 shadow-xl"
          aria-describedby={undefined}
        >
          <div className="mb-4 flex items-center justify-between">
            <Dialog.Title className="text-lg font-bold">{t('citation.title')}</Dialog.Title>
            <Dialog.Close asChild>
              <Button variant="ghost" aria-label={t('action.close')}>
                <X aria-hidden="true" className="h-5 w-5" />
              </Button>
            </Dialog.Close>
          </div>
          {citation && (
            <div className="space-y-4">
              <p className="text-sm text-ink-muted">
                {citation.spanId} · {t('citation.page', { page: citation.page })}
              </p>
              <blockquote className="fc-border rounded-md border border-border bg-surface-raised p-3">
                <p className="text-sm italic">“{citation.quote}”</p>
              </blockquote>
              {citation.confirmed ? (
                <p className="inline-flex items-center gap-2 font-semibold text-success">
                  <CheckCircle2 aria-hidden="true" className="h-4 w-4" />
                  {t('citation.verified')}
                </p>
              ) : (
                <div className="space-y-1">
                  <p className="inline-flex items-center gap-2 font-semibold text-ink-muted">
                    <CircleSlash aria-hidden="true" className="h-4 w-4" />
                    {t('citation.notConfirmed')}
                  </p>
                  <p className="text-sm text-ink-muted">{t('citation.notConfirmedHelp')}</p>
                </div>
              )}
            </div>
          )}
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
