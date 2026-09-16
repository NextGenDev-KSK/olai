import { act } from '@testing-library/react';
import { axe } from 'vitest-axe';
import { useStore } from '@/store/useStore';

/** Run axe scoped to the WCAG 2.x A/AA success criteria (what Olai targets). */
export function checkA11y(container: HTMLElement) {
  return axe(container, {
    runOnly: { type: 'tag', values: ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa'] },
  });
}

/** Reset the Zustand store to initial state between tests. */
export function resetStore(): void {
  useStore.setState({
    screen: 'onboarding',
    language: 'en',
    sessionId: null,
    demoScenario: null,
    docs: [],
    analysis: null,
    hasApiKey: false,
    loading: false,
    error: null,
    activeCitation: null,
    activeConflict: 0,
    qaHistory: [],
    reply: null,
    easyRead: false,
  });
}

/** Load a demo scenario through the store (via the offline mock backend). */
export async function loadDemo(scenario: 'eviction' | 'scam'): Promise<void> {
  await act(async () => {
    await useStore.getState().loadDemo(scenario);
  });
}
