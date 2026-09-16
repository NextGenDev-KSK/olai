import { useState } from 'react';
import { Button } from '@/components/Button';
import { CitationChip } from '@/components/CitationChip';
import { ListenButton } from '@/components/ListenButton';
import { StatusPill } from '@/components/StatusPill';
import { useTranslation } from '@/i18n/useTranslation';
import { useStore } from '@/store/useStore';

/** Grounded Q&A: answers come only from the documents, with status pills. */
export function AskPanel() {
  const { t } = useTranslation();
  const ask = useStore((s) => s.ask);
  const history = useStore((s) => s.qaHistory);
  const loading = useStore((s) => s.loading);
  const [question, setQuestion] = useState('');

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const q = question.trim();
    if (!q) return;
    setQuestion('');
    void ask(q);
  };

  return (
    <div className="space-y-4">
      <form onSubmit={onSubmit} className="space-y-2">
        <label htmlFor="ask-input" className="block font-semibold">
          {t('ask.title')}
        </label>
        <div className="flex flex-wrap gap-2">
          <input
            id="ask-input"
            value={question}
            onChange={(e) => setQuestion(e.target.value)}
            placeholder={t('ask.placeholder')}
            className="fc-border min-h-touch flex-1 rounded-md border border-border bg-surface px-3 py-2"
          />
          <Button type="submit" disabled={loading || question.trim() === ''}>
            {t('ask.submit')}
          </Button>
        </div>
        <p className="text-sm text-ink-muted">{t('ask.empty')}</p>
      </form>

      <ul aria-live="polite" className="space-y-3">
        {history.map((entry, i) => (
          <li key={i} className="fc-border rounded-lg border border-border p-3">
            <p className="font-medium">{entry.question}</p>
            <div className="mt-1 flex items-center gap-2">
              <StatusPill status={entry.answer.status} />
              <ListenButton text={entry.answer.text} />
            </div>
            <p className="mt-2 text-sm">{entry.answer.text}</p>
            {entry.answer.citations.length > 0 && (
              <div className="mt-2 flex flex-wrap gap-1">
                {entry.answer.citations.map((c) => (
                  <CitationChip key={`${c.spanId}:${c.quote}`} citation={c} />
                ))}
              </div>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
