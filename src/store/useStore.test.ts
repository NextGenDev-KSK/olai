import { describe, it, expect, beforeEach } from 'vitest';
import { act } from '@testing-library/react';
import { useStore } from './useStore';
import { resetStore } from '@/test/utils';

describe('useStore', () => {
  beforeEach(resetStore);

  it('loads the eviction demo into the dashboard', async () => {
    await act(async () => {
      await useStore.getState().loadDemo('eviction');
    });
    const s = useStore.getState();
    expect(s.screen).toBe('dashboard');
    expect(s.analysis?.conflicts).toHaveLength(3);
    expect(s.analysis?.tier.tier).toBe('needsLawyer');
  });

  it('appends grounded answers to history', async () => {
    await act(async () => {
      await useStore.getState().loadDemo('eviction');
      await useStore.getState().ask('What is the notice period?');
    });
    const history = useStore.getState().qaHistory;
    expect(history).toHaveLength(1);
    expect(history[0]?.answer.status).toBe('answered');
  });

  it('drafts a reply', async () => {
    await act(async () => {
      await useStore.getState().loadDemo('eviction');
      await useStore.getState().generateReply();
    });
    expect(useStore.getState().reply?.text).toContain('acknowledge');
  });

  it('opens and closes the citation drawer', () => {
    act(() => {
      useStore.getState().openCitation({ spanId: 'D1-P1-S1', quote: 'x', page: 1, similarity: 1, confirmed: true });
    });
    expect(useStore.getState().activeCitation).not.toBeNull();
    act(() => useStore.getState().closeCitation());
    expect(useStore.getState().activeCitation).toBeNull();
  });

  it('deletes everything and returns to onboarding', async () => {
    await act(async () => {
      await useStore.getState().loadDemo('eviction');
      await useStore.getState().deleteEverything();
    });
    const s = useStore.getState();
    expect(s.screen).toBe('onboarding');
    expect(s.analysis).toBeNull();
    expect(s.docs).toHaveLength(0);
  });

  it('stores an API key and toggles easy-read', async () => {
    await act(async () => {
      await useStore.getState().saveApiKey('sk-ant-test');
    });
    expect(useStore.getState().hasApiKey).toBe(true);
    act(() => useStore.getState().toggleEasyRead());
    expect(useStore.getState().easyRead).toBe(true);
  });

  it('ask and generateReply are no-ops without a session', async () => {
    await act(async () => {
      await useStore.getState().ask('q');
      await useStore.getState().generateReply();
    });
    expect(useStore.getState().qaHistory).toHaveLength(0);
    expect(useStore.getState().reply).toBeNull();
  });

  it('setLanguage works with and without an active session', async () => {
    await act(async () => {
      await useStore.getState().setLanguage('hi');
    });
    expect(useStore.getState().language).toBe('hi');
    await act(async () => {
      await useStore.getState().loadDemo('eviction');
      await useStore.getState().setLanguage('ta');
    });
    expect(useStore.getState().language).toBe('ta');
  });

  it('init and simple setters update state', async () => {
    await act(async () => {
      await useStore.getState().init();
    });
    expect(useStore.getState().hasApiKey).toBe(false);
    act(() => {
      useStore.getState().setActiveConflict(2);
      useStore.getState().goToScreen('dashboard');
      useStore.getState().clearError();
    });
    expect(useStore.getState().activeConflict).toBe(2);
    expect(useStore.getState().screen).toBe('dashboard');
  });
});
