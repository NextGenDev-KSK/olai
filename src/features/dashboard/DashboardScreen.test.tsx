import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import App from '@/App';
import { resetStore, loadDemo, checkA11y } from '@/test/utils';

describe('DashboardScreen (eviction demo)', () => {
  beforeEach(async () => {
    resetStore();
    await loadDemo('eviction');
  });

  it('shows the help-tier banner, paper card, and computed deadline', () => {
    render(<App />);
    expect(screen.getByRole('heading', { name: /needs a lawyer/i })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: /what this document is/i })).toBeInTheDocument();
    expect(screen.getByText('2026-09-17')).toBeInTheDocument();
    expect(screen.getByText(/2026-09-10 \+ 7 days/i)).toBeInTheDocument();
  });

  it('lists three numbered conflicts with severity words', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('tab', { name: 'Conflicts' }));
    expect(screen.getByText(/3 conflict/i)).toBeInTheDocument();
    expect(screen.getAllByText(/Severity: High/i).length).toBeGreaterThan(0);
  });

  it('opens the compare view and reports the conflict position', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('tab', { name: 'Conflicts' }));
    await userEvent.click(screen.getAllByRole('button', { name: /see side by side/i })[0]!);
    expect(await screen.findByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText(/Conflict 1 of 3/i)).toBeInTheDocument();
  });

  it('answers a grounded question with a status pill', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('tab', { name: 'Ask' }));
    await userEvent.type(screen.getByLabelText(/ask about your documents/i), 'notice period?');
    await userEvent.click(screen.getByRole('button', { name: 'Ask' }));
    expect(await screen.findByText(/one month/i)).toBeInTheDocument();
    expect(screen.getByText('Answered')).toBeInTheDocument();
  });

  it('drafts a neutral reply', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('tab', { name: 'Actions' }));
    await userEvent.click(screen.getByRole('button', { name: /draft a neutral reply/i }));
    expect(await screen.findByText(/acknowledge receipt/i)).toBeInTheDocument();
  });

  it('opens the citation drawer from a citation chip', async () => {
    render(<App />);
    await userEvent.click(screen.getAllByRole('button', { name: /view citation/i })[0]!);
    expect(await screen.findByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText(/EVICTION NOTICE/)).toBeInTheDocument();
  });

  it('returns to onboarding on delete everything', async () => {
    vi.spyOn(window, 'confirm').mockReturnValue(true);
    render(<App />);
    await userEvent.click(screen.getByRole('button', { name: /delete everything/i }));
    expect(await screen.findByRole('heading', { name: /welcome to olai/i })).toBeInTheDocument();
  });

  it('exports calendar reminders as .ics', async () => {
    const spy = vi.spyOn(URL, 'createObjectURL');
    render(<App />);
    await userEvent.click(screen.getByRole('tab', { name: 'Actions' }));
    await userEvent.click(screen.getByRole('button', { name: /add reminders/i }));
    expect(spy).toHaveBeenCalled();
  });

  it('exports the drafted reply as .txt', async () => {
    const spy = vi.spyOn(URL, 'createObjectURL');
    render(<App />);
    await userEvent.click(screen.getByRole('tab', { name: 'Actions' }));
    await userEvent.click(screen.getByRole('button', { name: /draft a neutral reply/i }));
    await screen.findByText(/acknowledge receipt/i);
    await userEvent.click(screen.getByRole('button', { name: /export reply/i }));
    expect(spy).toHaveBeenCalled();
  });

  it('navigates conflicts with the J key in compare', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('tab', { name: 'Conflicts' }));
    await userEvent.click(screen.getAllByRole('button', { name: /see side by side/i })[0]!);
    await screen.findByRole('dialog');
    await userEvent.keyboard('j');
    expect(screen.getByText(/Conflict 2 of 3/i)).toBeInTheDocument();
  });

  it('has no accessibility violations', async () => {
    const { container } = render(<App />);
    expect(await checkA11y(container)).toHaveNoViolations();
  });
});
