/**
 * Display-side PII masking for previews and exports, mirroring the Rust
 * detector. The backend masks logs; this masks what the user sees and exports.
 */

interface Rule {
  re: RegExp;
  mask: (match: string) => string;
}

function keepLast(match: string, keep: number): string {
  const chars = match.replace(/[^A-Za-z0-9]/g, '');
  if (chars.length <= keep) return 'X'.repeat(Math.max(chars.length, 1));
  return 'X'.repeat(chars.length - keep) + chars.slice(chars.length - keep);
}

function maskHandle(match: string): string {
  const parts = match.split('@');
  const local = parts[0] ?? '';
  const domain = parts[1] ?? '';
  return `${local[0] ?? 'x'}***@${domain}`;
}

// Ordered by priority; earlier rules win because later ones run on already-masked text.
const RULES: Rule[] = [
  { re: /[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+\.[A-Za-z]{2,}/g, mask: maskHandle }, // email
  { re: /[A-Za-z0-9.\-_]{2,}@[A-Za-z]{2,}/g, mask: maskHandle }, // UPI
  {
    re: /\b\d{4}\s\d{4}\s\d{4}\b|\b\d{12}\b/g,
    mask: (m) => `XXXX-XXXX-${keepLast(m, 4).slice(-4)}`,
  }, // Aadhaar
  { re: /\b[A-Z]{5}\d{4}[A-Z]\b/g, mask: (m) => keepLast(m, 4) }, // PAN
  { re: /\b\d{11,18}\b/g, mask: (m) => keepLast(m, 4) }, // bank account
  { re: /(?:\+?91[\s-]?)?\b[6-9]\d(?:[\s-]?\d){8}\b/g, mask: (m) => keepLast(m, 4) }, // phone
];

/** Return `text` with detected PII masked (keeping the last few characters). */
export function maskPii(text: string): string {
  let out = text;
  for (const rule of RULES) {
    out = out.replace(rule.re, (m) => rule.mask(m));
  }
  return out;
}
