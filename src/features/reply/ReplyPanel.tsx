import { PenLine, Download } from 'lucide-react';
import { Button } from '@/components/Button';
import { CitationChip } from '@/components/CitationChip';
import { ListenButton } from '@/components/ListenButton';
import { useTranslation } from '@/i18n/useTranslation';
import { useStore } from '@/store/useStore';
import { buildReplyTxt, downloadText } from '@/lib/export';
import { maskPii } from '@/lib/pii';

/** Draft-reply panel: generate a neutral reply, listen, and export to .txt. */
export function ReplyPanel() {
  const { t } = useTranslation();
  const reply = useStore((s) => s.reply);
  const generate = useStore((s) => s.generateReply);
  const loading = useStore((s) => s.loading);

  const onExport = () => {
    if (reply) {
      downloadText('olai-reply.txt', buildReplyTxt(reply, t('app.disclaimer')), 'text/plain');
    }
  };

  return (
    <section aria-labelledby="reply-heading" className="fc-border rounded-lg border border-border p-4">
      <h3 id="reply-heading" className="inline-flex items-center gap-2 font-semibold">
        <PenLine aria-hidden="true" className="h-4 w-4" />
        {t('reply.title')}
      </h3>
      <p className="mt-1 text-sm text-ink-muted">{t('reply.intro')}</p>
      <div className="mt-2">
        <Button onClick={() => void generate()} disabled={loading}>
          {t('action.generateReply')}
        </Button>
      </div>
      {reply ? (
        <div className="mt-3 space-y-2">
          <p className="fc-border whitespace-pre-wrap rounded-md border border-border bg-surface-raised p-3 text-sm">
            {maskPii(reply.text)}
          </p>
          {reply.footnotes.length > 0 && (
            <div>
              <p className="text-sm font-semibold">{t('reply.footnotes')}</p>
              <div className="mt-1 flex flex-wrap gap-1">
                {reply.footnotes.map((c) => (
                  <CitationChip key={`${c.spanId}:${c.quote}`} citation={c} />
                ))}
              </div>
            </div>
          )}
          <div className="flex flex-wrap gap-2">
            <ListenButton text={reply.text} />
            <Button variant="secondary" onClick={onExport}>
              <Download aria-hidden="true" className="h-4 w-4" />
              {t('action.exportReply')}
            </Button>
          </div>
        </div>
      ) : (
        <p className="mt-2 text-sm text-ink-muted">{t('reply.empty')}</p>
      )}
    </section>
  );
}
