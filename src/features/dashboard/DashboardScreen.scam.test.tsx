import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import App from '@/App';
import { resetStore, loadDemo, checkA11y } from '@/test/utils';

describe('DashboardScreen (scam demo)', () => {
  beforeEach(async () => {
    resetStore();
    await loadDemo('scam');
  });

  it('escalates to needs-a-lawyer and surfaces flagged text', () => {
    render(<App />);
    expect(screen.getByRole('heading', { name: /needs a lawyer/i })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: /flagged text/i })).toBeInTheDocument();
    expect(screen.getByText(/hidden or zero-width characters/i)).toBeInTheDocument();
  });

  it('shows no conflicts when there is no agreement', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('tab', { name: 'Conflicts' }));
    expect(screen.getByText(/no conflicts were found/i)).toBeInTheDocument();
  });

  it('has no accessibility violations', async () => {
    const { container } = render(<App />);
    expect(await checkA11y(container)).toHaveNoViolations();
  });
});
