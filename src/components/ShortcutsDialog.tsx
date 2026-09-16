import * as Dialog from '@radix-ui/react-dialog';
import { X } from 'lucide-react';
import { Button } from './Button';
import { useTranslation } from '@/i18n/useTranslation';

const ROWS: { keys: string; labelKey: 'shortcuts.help' | 'shortcuts.tabs' | 'shortcuts.conflicts' | 'shortcuts.close' }[] = [
  { keys: '?', labelKey: 'shortcuts.help' },
  { keys: '← →', labelKey: 'shortcuts.tabs' },
  { keys: 'J / K', labelKey: 'shortcuts.conflicts' },
  { keys: 'Esc', labelKey: 'shortcuts.close' },
];

/** A keyboard-shortcuts help dialog, opened with `?`. */
export function ShortcutsDialog({ open, onOpenChange }: { open: boolean; onOpenChange: (o: boolean) => void }) {
  const { t } = useTranslation();
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/40" />
        <Dialog.Content
          className="fc-border fixed left-1/2 top-1/2 z-50 w-full max-w-sm -translate-x-1/2 -translate-y-1/2 rounded-lg border border-border bg-surface p-5"
          aria-describedby={undefined}
        >
          <div className="mb-3 flex items-center justify-between">
            <Dialog.Title className="text-lg font-bold">{t('shortcuts.title')}</Dialog.Title>
            <Dialog.Close asChild>
              <Button variant="ghost" aria-label={t('action.close')}>
                <X aria-hidden="true" className="h-5 w-5" />
              </Button>
            </Dialog.Close>
          </div>
          <dl className="space-y-2">
            {ROWS.map((r) => (
              <div key={r.keys} className="flex items-center justify-between gap-4">
                <dt>
                  <kbd className="fc-border rounded border border-border bg-surface-raised px-2 py-1 text-sm">
                    {r.keys}
                  </kbd>
                </dt>
                <dd className="text-sm">{t(r.labelKey)}</dd>
              </div>
            ))}
          </dl>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
