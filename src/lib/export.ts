/**
 * Client-side exports: calendar reminders (.ics) and the reply draft (.txt).
 * Dates come already-computed from Rust; this only formats them.
 */
import type { Deadline, ReplyDraft } from '@/lib/bindings';
import { maskPii } from './pii';

function icsDate(iso: string): string {
  return iso.replace(/-/g, '');
}

function escapeIcs(text: string): string {
  return text.replace(/([,;\\])/g, '\\$1').replace(/\n/g, '\\n');
}

/** Build an RFC 5545 VCALENDAR with an all-day event + 1-day alarm per deadline. */
export function buildIcs(deadlines: Deadline[]): string {
  const lines = ['BEGIN:VCALENDAR', 'VERSION:2.0', 'PRODID:-//Olai//Reminders//EN', 'CALSCALE:GREGORIAN'];
  for (const [i, d] of deadlines.entries()) {
    if (!d.date) continue;
    const date = icsDate(d.date);
    lines.push(
      'BEGIN:VEVENT',
      `UID:olai-${i}-${date}@olai.app`,
      `DTSTART;VALUE=DATE:${date}`,
      `SUMMARY:${escapeIcs(`Olai reminder: ${d.label}`)}`,
      'BEGIN:VALARM',
      'TRIGGER:-P1D',
      'ACTION:DISPLAY',
      `DESCRIPTION:${escapeIcs(d.label)}`,
      'END:VALARM',
      'END:VEVENT',
    );
  }
  lines.push('END:VCALENDAR');
  return lines.join('\r\n');
}

/** Build the plain-text reply export, with PII masked and a disclaimer. */
export function buildReplyTxt(reply: ReplyDraft, disclaimer: string): string {
  const body = maskPii(reply.text);
  const refs =
    reply.footnotes.length > 0
      ? '\n\nReferences:\n' +
        reply.footnotes.map((f, i) => `[${i + 1}] ${f.spanId}: "${maskPii(f.quote)}"`).join('\n')
      : '';
  return `${body}${refs}\n\n---\n${disclaimer}\n`;
}

/** Trigger a browser download of text content. */
export function downloadText(fileName: string, content: string, mime: string): void {
  const blob = new Blob([content], { type: mime });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.download = fileName;
  document.body.appendChild(anchor);
  anchor.click();
  document.body.removeChild(anchor);
  URL.revokeObjectURL(url);
}
