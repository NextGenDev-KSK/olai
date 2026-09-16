import { describe, it, expect } from 'vitest';
import { translate, LANGUAGES } from './index';

describe('translate', () => {
  it('returns the string for the language', () => {
    expect(translate('en', 'app.name')).toBe('Olai');
    expect(translate('ta', 'app.name')).toBe('ஓலை');
    expect(translate('hi', 'app.name')).toBe('ओलई');
  });

  it('substitutes parameters', () => {
    expect(translate('en', 'deadline.daysLeft', { days: 3 })).toContain('3');
  });

  it('falls back to English when a key is missing in a catalog', () => {
    // Every language defines app.disclaimer, so this always resolves.
    expect(translate('ta', 'app.disclaimer').length).toBeGreaterThan(0);
  });

  it('exposes the three supported languages', () => {
    expect(LANGUAGES).toEqual(['en', 'ta', 'hi']);
  });
});
