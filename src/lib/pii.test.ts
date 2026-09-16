import { describe, it, expect } from 'vitest';
import { maskPii } from './pii';

describe('maskPii', () => {
  it('masks an Aadhaar-like number keeping the last four', () => {
    const out = maskPii('Aadhaar 1234 5678 9012 on file');
    expect(out).toContain('XXXX-XXXX-9012');
    expect(out).not.toContain('1234 5678 9012');
  });

  it('masks an email local part', () => {
    const out = maskPii('write to jane.doe@example.com');
    expect(out).toContain('@example.com');
    expect(out).not.toContain('jane.doe@example.com');
  });

  it('masks a UPI id but keeps the handle', () => {
    const out = maskPii('pay 9876543210@ybl now');
    expect(out).toContain('@ybl');
    expect(out).not.toContain('9876543210@ybl');
  });

  it('masks a phone number keeping the last four', () => {
    const out = maskPii('call 9876543210 today');
    expect(out).toContain('3210');
    expect(out).not.toContain('9876543210');
  });

  it('leaves clean text unchanged', () => {
    expect(maskPii('vacate within 7 days')).toBe('vacate within 7 days');
  });
});
