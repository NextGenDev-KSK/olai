import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import App from '@/App';
import { resetStore, checkA11y } from '@/test/utils';

describe('OnboardingScreen', () => {
  beforeEach(resetStore);

  it('renders the welcome, privacy, and demo entry points', () => {
    render(<App />);
    expect(screen.getByRole('heading', { name: /welcome to olai/i })).toBeInTheDocument();
    expect(screen.getByText(/never saved to disk/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /eviction notice demo/i })).toBeInTheDocument();
  });

  it('has no accessibility violations', async () => {
    const { container } = render(<App />);
    expect(await checkA11y(container)).toHaveNoViolations();
  });

  it('loads the demo when a demo button is pressed', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('button', { name: /eviction notice demo/i }));
    expect(await screen.findByRole('heading', { name: /needs a lawyer/i })).toBeInTheDocument();
  });

  it('shows the persistent disclaimer footer', () => {
    render(<App />);
    expect(screen.getByText(/it is not a lawyer/i)).toBeInTheDocument();
  });

  it('saves an API key and confirms it', async () => {
    render(<App />);
    await userEvent.type(screen.getByLabelText('Anthropic API key'), 'sk-ant-demo');
    await userEvent.click(screen.getByRole('button', { name: 'Save key' }));
    expect(await screen.findByText(/key saved securely/i)).toBeInTheDocument();
  });

  it('loads the scam demo', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('button', { name: /scam notice demo/i }));
    expect(await screen.findByRole('heading', { name: /flagged text/i })).toBeInTheDocument();
  });
});
