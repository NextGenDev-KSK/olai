import { create } from 'zustand';
import type {
  Analysis,
  Citation,
  DemoScenarioKind,
  Document,
  Language,
  QaAnswer,
  ReplyDraft,
  UserFacingError,
} from '@/lib/bindings';
import { ipc, toUserError } from '@/lib/ipc';

/** Which top-level screen is shown. */
export type Screen = 'onboarding' | 'dashboard';

/** A question and its grounded answer. */
export interface QaEntry {
  question: string;
  answer: QaAnswer;
}

interface State {
  screen: Screen;
  language: Language;
  sessionId: string | null;
  demoScenario: DemoScenarioKind | null;
  docs: Document[];
  analysis: Analysis | null;
  hasApiKey: boolean;
  loading: boolean;
  error: UserFacingError | null;
  activeCitation: Citation | null;
  activeConflict: number;
  qaHistory: QaEntry[];
  reply: ReplyDraft | null;
  easyRead: boolean;

  init: () => Promise<void>;
  setLanguage: (language: Language) => Promise<void>;
  saveApiKey: (key: string) => Promise<void>;
  loadDemo: (scenario: DemoScenarioKind) => Promise<void>;
  ask: (question: string) => Promise<void>;
  generateReply: () => Promise<void>;
  deleteEverything: () => Promise<void>;
  openCitation: (citation: Citation) => void;
  closeCitation: () => void;
  setActiveConflict: (index: number) => void;
  goToScreen: (screen: Screen) => void;
  toggleEasyRead: () => void;
  clearError: () => void;
}

export const useStore = create<State>((set, get) => ({
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

  init: async () => {
    try {
      set({ hasApiKey: await ipc.hasApiKey() });
    } catch {
      /* offline: leave hasApiKey false */
    }
  },

  setLanguage: async (language) => {
    set({ language });
    const { sessionId } = get();
    if (sessionId) {
      try {
        await ipc.setLanguage(sessionId, language);
      } catch (err) {
        set({ error: toUserError(err) });
      }
    }
  },

  saveApiKey: async (key) => {
    set({ loading: true, error: null });
    try {
      await ipc.setApiKey(key);
      set({ hasApiKey: true });
    } catch (err) {
      set({ error: toUserError(err) });
    } finally {
      set({ loading: false });
    }
  },

  loadDemo: async (scenario) => {
    set({ loading: true, error: null });
    try {
      const demo = await ipc.loadDemo(scenario);
      set({
        sessionId: demo.sessionId,
        docs: demo.docs,
        analysis: demo.analysis,
        demoScenario: scenario,
        screen: 'dashboard',
        activeConflict: 0,
        qaHistory: [],
        reply: null,
      });
    } catch (err) {
      set({ error: toUserError(err) });
    } finally {
      set({ loading: false });
    }
  },

  ask: async (question) => {
    const { sessionId } = get();
    if (!sessionId || !question.trim()) return;
    set({ loading: true, error: null });
    try {
      const answer = await ipc.ask(sessionId, question);
      set((s) => ({ qaHistory: [...s.qaHistory, { question, answer }] }));
    } catch (err) {
      set({ error: toUserError(err) });
    } finally {
      set({ loading: false });
    }
  },

  generateReply: async () => {
    const { sessionId } = get();
    if (!sessionId) return;
    set({ loading: true, error: null });
    try {
      set({ reply: await ipc.draftReply(sessionId) });
    } catch (err) {
      set({ error: toUserError(err) });
    } finally {
      set({ loading: false });
    }
  },

  deleteEverything: async () => {
    try {
      await ipc.deleteEverything();
    } catch {
      /* clearing local state is what matters */
    }
    set({
      screen: 'onboarding',
      sessionId: null,
      demoScenario: null,
      docs: [],
      analysis: null,
      activeCitation: null,
      activeConflict: 0,
      qaHistory: [],
      reply: null,
      error: null,
    });
  },

  openCitation: (citation) => set({ activeCitation: citation }),
  closeCitation: () => set({ activeCitation: null }),
  setActiveConflict: (index) => set({ activeConflict: index }),
  goToScreen: (screen) => set({ screen }),
  toggleEasyRead: () => set((s) => ({ easyRead: !s.easyRead })),
  clearError: () => set({ error: null }),
}));
