import { test, expect } from '@playwright/test';

// A keyboard-only journey: skip link, activating the demo with Enter, the
// shortcuts dialog (?), and arrow-key tab navigation. No mouse is used to
// operate controls (only page.goto and focus/keyboard).
test('keyboard-only: skip link, demo via Enter, shortcuts dialog, arrow-key tabs', async ({
  page,
}) => {
  await page.goto('/');
  await expect(page.getByRole('heading', { name: /welcome to olai/i })).toBeVisible();

  // The first Tab lands on the skip link.
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: /skip to main content/i })).toBeFocused();

  // Activate the eviction demo using the keyboard (focus + Enter).
  const demo = page.getByRole('button', { name: /eviction notice demo/i });
  await demo.focus();
  await expect(demo).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.getByRole('heading', { name: /needs a lawyer/i })).toBeVisible();

  // The shortcuts dialog opens with "?" and closes with Escape.
  await page.keyboard.press('?');
  await expect(page.getByRole('heading', { name: /keyboard shortcuts/i })).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(page.getByRole('heading', { name: /keyboard shortcuts/i })).toBeHidden();

  // Arrow keys move between dashboard tabs (Radix roving tabindex).
  const overview = page.getByRole('tab', { name: 'Overview' });
  await overview.focus();
  await page.keyboard.press('ArrowRight');
  await expect(page.getByRole('tab', { name: 'Conflicts' })).toHaveAttribute(
    'aria-selected',
    'true',
  );
});
