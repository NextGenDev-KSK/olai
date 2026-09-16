/**
 * Typed IPC client. In the Tauri app it invokes Rust commands; in a plain
 * browser (dev, Vitest, Playwright) it falls back to the offline mock backend.
 * All document data and secrets stay in Rust — this layer only passes messages.
 */
import type {
  Analysis,
  DemoScenarioKind,
  DemoSession,
  Document,
  IngestFile,
  Language,
  QaAnswer,
  ReplyDraft,
  UserFacingError,
} from '@/lib/bindings';
import { mockInvoke } from './mock-backend';

function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri()) {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke<T>(cmd, args);
  }
  return mockInvoke<T>(cmd, args);
}

/** Narrow an unknown rejection into a `UserFacingError`. */
export function toUserError(err: unknown): UserFacingError {
  if (err && typeof err === 'object' && 'code' in err) {
    return err as UserFacingError;
  }
  return { code: 'error.internal', detail: null };
}

/** The typed command surface. */
export const ipc = {
  hasApiKey: () => invoke<boolean>('has_api_key'),
  setApiKey: (key: string) => invoke<null>('set_api_key', { key }),
  deleteApiKey: () => invoke<null>('delete_api_key'),
  startSession: (language: Language) => invoke<string>('start_session', { language }),
  setLanguage: (session: string, language: Language) =>
    invoke<null>('set_language', { session, language }),
  setReceivedDate: (session: string, date: string | null) =>
    invoke<null>('set_received_date', { session, date }),
  ingestFiles: (session: string, files: IngestFile[]) =>
    invoke<Document[]>('ingest_files', { session, files }),
  runAnalysis: (session: string) => invoke<Analysis>('run_analysis', { session }),
  getAnalysis: (session: string) => invoke<Analysis>('get_analysis', { session }),
  ask: (session: string, question: string) => invoke<QaAnswer>('ask', { session, question }),
  draftReply: (session: string) => invoke<ReplyDraft>('draft_reply', { session }),
  loadDemo: (scenario: DemoScenarioKind) => invoke<DemoSession>('load_demo', { scenario }),
  deleteEverything: () => invoke<null>('delete_everything'),
};
