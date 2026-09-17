import { test, expect } from '@playwright/test';

// The full demo journey against the built app with the offline mock IPC layer.
test('demo flow: open → dashboard → compare → ask → export .ics → delete', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('heading', { name: /welcome to olai/i })).toBeVisible();

  await page.getByRole('button', { name: /eviction notice demo/i }).click();
  await expect(page.getByRole('heading', { name: /needs a lawyer/i })).toBeVisible();
  // The countdown shows the computed date and its status (e.g. "…-09-17· Due today").
  await expect(page.getByText(/2026-09-17.*Due today/i)).toBeVisible();
  // Expand the transparency disclosure to reveal how the deadline was derived.
  await page.getByText(/how this date was worked out/i).click();
  // The calculation trace shows the arithmetic done in code (never by the model).
  await expect(page.getByText(/2026-09-10 \+ 7 days = 2026-09-17/i)).toBeVisible();

  // Conflicts + compare with keyboard navigation.
  await page.getByRole('tab', { name: 'Conflicts' }).click();
  await expect(page.getByText(/3 conflict/i)).toBeVisible();
  await page
    .getByRole('button', { name: /see side by side/i })
    .first()
    .click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await expect(page.getByText(/Conflict 1 of 3/i)).toBeVisible();
  await page.keyboard.press('j');
  await expect(page.getByText(/Conflict 2 of 3/i)).toBeVisible();
  await page.getByRole('button', { name: 'Close' }).click();

  // Grounded question.
  await page.getByRole('tab', { name: 'Ask' }).click();
  await page.getByLabel(/ask about your documents/i).fill('What is the notice period?');
  await page.getByRole('button', { name: 'Ask' }).click();
  await expect(page.getByText(/one month/i)).toBeVisible();
  await expect(page.getByText('Answered')).toBeVisible();

  // Export reminders as .ics.
  await page.getByRole('tab', { name: 'Actions' }).click();
  const [download] = await Promise.all([
    page.waitForEvent('download'),
    page.getByRole('button', { name: /add reminders/i }).click(),
  ]);
  expect(download.suggestedFilename()).toBe('olai-reminders.ics');

  // Delete everything returns to onboarding.
  page.on('dialog', (d) => d.accept());
  await page.getByRole('button', { name: /delete everything/i }).click();
  await expect(page.getByRole('heading', { name: /welcome to olai/i })).toBeVisible();
});
