import { describe, it, expect, vi } from 'vitest';
import type { Deadline, ReplyDraft } from '@/lib/bindings';
import { buildIcs, buildReplyTxt, downloadText } from './export';

function deadline(date: string | null): Deadline {
  return {
    label: 'Vacate the premises',
    date,
    daysRemaining: 1,
    calc: { anchor: '2026-09-10', steps: ['2026-09-10 + 7 days = 2026-09-17'], needsAnchor: false },
    source: { id: 's', text: 'x', citations: [], status: 'verified' },
  };
}

describe('buildIcs', () => {
  it('emits a VEVENT with an alarm for a dated deadline', () => {
    const ics = buildIcs([deadline('2026-09-17')]);
    expect(ics).toContain('BEGIN:VCALENDAR');
    expect(ics).toContain('DTSTART;VALUE=DATE:20260917');
    expect(ics).toContain('SUMMARY:Olai reminder: Vacate the premises');
    expect(ics).toContain('BEGIN:VALARM');
  });

  it('skips deadlines without a date', () => {
    const ics = buildIcs([deadline(null)]);
    expect(ics).not.toContain('BEGIN:VEVENT');
  });
});

describe('buildReplyTxt', () => {
  it('masks PII and appends the disclaimer', () => {
    const reply: ReplyDraft = {
      text: 'Contact me at jane@example.com.',
      footnotes: [{ spanId: 'D1-P1-S1', quote: 'seven days', page: 1, similarity: 1, confirmed: true }],
    };
    const txt = buildReplyTxt(reply, 'Not a lawyer.');
    expect(txt).toContain('Not a lawyer.');
    expect(txt).not.toContain('jane@example.com');
    expect(txt).toContain('References:');
  });
});

describe('downloadText', () => {
  it('creates and revokes an object URL', () => {
    const createSpy = vi.spyOn(URL, 'createObjectURL');
    const revokeSpy = vi.spyOn(URL, 'revokeObjectURL');
    downloadText('f.txt', 'hello', 'text/plain');
    expect(createSpy).toHaveBeenCalled();
    expect(revokeSpy).toHaveBeenCalled();
  });
});
