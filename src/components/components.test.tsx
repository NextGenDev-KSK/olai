import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { SeverityBadge } from './SeverityBadge';
import { StatusPill } from './StatusPill';
import { ClaimText } from './ClaimText';
import App from '@/App';
import { resetStore, loadDemo } from '@/test/utils';

describe('shared components', () => {
  beforeEach(resetStore);

  it('SeverityBadge pairs a word with an icon (not colour alone)', () => {
    render(<SeverityBadge severity="high" />);
    expect(screen.getByText(/Severity: High/i)).toBeInTheDocument();
  });

  it('StatusPill shows the status word', () => {
    render(<StatusPill status="needsLawyer" />);
    expect(screen.getByText('Needs a lawyer')).toBeInTheDocument();
  });

  it('ClaimText greys and labels an unconfirmed claim', () => {
    render(<ClaimText claim={{ id: 'x', text: 'maybe', citations: [], status: 'notConfirmed' }} />);
    expect(screen.getByText(/Not confirmed/i)).toBeInTheDocument();
  });

  it('switches the UI language to Tamil', async () => {
    render(<App />);
    await userEvent.selectOptions(screen.getByLabelText('Language'), 'ta');
    expect(screen.getByRole('heading', { name: /ஓலைக்கு/ })).toBeInTheDocument();
  });

  it('opens the keyboard shortcuts dialog with "?"', async () => {
    await loadDemo('eviction');
    render(<App />);
    await userEvent.keyboard('?');
    expect(await screen.findByRole('heading', { name: /keyboard shortcuts/i })).toBeInTheDocument();
  });
});
