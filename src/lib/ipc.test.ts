import { describe, it, expect } from 'vitest';
import { ipc, toUserError } from './ipc';

describe('toUserError', () => {
  it('passes through a UserFacingError-shaped object', () => {
    expect(toUserError({ code: 'error.timeout', detail: null }).code).toBe('error.timeout');
  });
  it('wraps unknown errors as internal', () => {
    expect(toUserError(new Error('x')).code).toBe('error.internal');
    expect(toUserError(undefined).code).toBe('error.internal');
  });
});

describe('ipc client (mock backend)', () => {
  it('routes mocked commands', async () => {
    expect(await ipc.hasApiKey()).toBe(false);
    await ipc.setApiKey('sk-ant-x');
    await ipc.deleteApiKey();
    await ipc.setLanguage('demo', 'ta');
    await ipc.setReceivedDate('demo', '2026-09-10');
    await ipc.deleteEverything();
    expect(await ipc.startSession('en')).toBe('mock-session');
    const demo = await ipc.loadDemo('eviction');
    expect(demo.analysis.conflicts).toHaveLength(3);
    expect((await ipc.ask('demo', 'q')).status).toBe('answered');
    expect((await ipc.draftReply('demo')).text.length).toBeGreaterThan(0);
    expect((await ipc.getAnalysis('demo')).conflicts).toHaveLength(3);
  });

  it('rejects unmocked commands', async () => {
    await expect(ipc.runAnalysis('demo')).rejects.toBeTruthy();
    await expect(ipc.ingestFiles('demo', [])).rejects.toBeTruthy();
  });
});
