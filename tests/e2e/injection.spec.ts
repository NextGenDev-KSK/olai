import { test, expect } from '@playwright/test';

// The injection-document flow: the scam demo contains a prompt-injection /
// hidden-text attempt. Olai must treat it as data, surface it as a flagged
// finding, and escalate to "needs a lawyer".
test('injection document: scam demo flags hidden/instruction text and escalates', async ({
  page,
}) => {
  await page.goto('/');
  await page.getByRole('button', { name: /scam notice demo/i }).click();

  // Scam signals force the highest help tier.
  await expect(page.getByRole('heading', { name: /needs a lawyer/i })).toBeVisible();

  // The Overview tab surfaces the flagged-text panel.
  await expect(page.getByRole('heading', { name: /flagged text/i })).toBeVisible();
  await expect(page.getByText(/hidden characters or instructions aimed at the ai/i)).toBeVisible();

  // The "not a lawyer" disclaimer is always present.
  await expect(page.getByText(/it is not a lawyer and does not give legal advice/i)).toBeVisible();
});
