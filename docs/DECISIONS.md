# Architectural Decision Log

Chronological, terse records of non‑obvious choices. Each entry: context → decision →
consequence.

## D1 — Shared types via `ts-rs`
**Context:** the frontend must not hand‑duplicate backend shapes.
**Decision:** derive `ts_rs::TS` on every domain type; export to `src/lib/bindings/`; a
`cargo test export_bindings` regenerates them and CI treats drift as a failure.
**Consequence:** one source of truth (Rust), zero runtime cost, TS stays in lockstep.

## D2 — Rust owns all secrets, network, and arithmetic
**Decision:** the API key lives only in Windows Credential Manager; all HTTP happens in
Rust (`reqwest` + `rustls`); date/money math is done in `services/dates.rs` (`chrono`).
**Consequence:** the WebView2 frontend never sees the key and never computes deadlines;
the strict CSP can forbid all remote origins.

## D3 — Documents held in Rust memory; frontend gets verified view‑models
**Decision:** ingested documents live in an in‑memory `AppState` keyed by session; the
frontend receives only derived, PII‑masked, citation‑verified results.
**Consequence:** centralizes the grounding gate and PII masking; “Delete everything”
is a single `state.delete_all()`; nothing hits disk.

## D4 — Demo mode = `MockProvider` + compiled‑in synthetic fixtures
**Decision:** `providers::mock::MockProvider` replays recorded responses for bundled
synthetic documents (`fixtures/`); the browser build uses `src/lib/mock-backend.ts`.
**Consequence:** the whole pipeline (and the e2e suite) runs offline and
deterministically, with no API key.

## D5 — Citation verification is a hard gate in Rust
**Decision:** every model claim must cite spans; `services/verification.rs` normalizes and
fuzzy‑matches the quote to the stored span (Levenshtein substring, similarity ≥ 0.9, ≤ 25
words). Failures ⇒ `NotConfirmed`, rendered greyed out.
**Consequence:** fabricated or hallucinated quotes cannot be presented as fact
(`fabricated_citation_is_marked_not_confirmed`).

## D6 — Optional Tauri via a default `app` feature
**Context:** compiling the full WebView/wry stack for every unit‑test run is slow and can
mask pure‑logic failures.
**Decision:** `tauri`, `reqwest`, and `keyring` are optional, enabled by the default `app`
feature (and an `http` feature for the real provider). `cargo test --no-default-features`
exercises all pure logic in seconds; `commands`/`secrets`/`anthropic` are `#[cfg]`‑gated.
**Consequence:** fast, reliable core tests; the real app build is unchanged.

## D7 — Model tiering
**Decision:** `providers::ModelTier { Fast, Smart }` — cheaper/faster model for simple
extraction and Q&A, the more capable model for comparison and drafting.
**Consequence:** lower cost/latency without weakening the hard steps.

## D8 — Schema‑validated prompts with one repair retry
**Decision:** prompts live in versioned files (`prompts/*.v1.md`) each with a JSON schema;
model output is parsed/validated, and on failure retried once with a repair message, then
surfaced as `SchemaViolation`.
**Consequence:** robust to minor model formatting drift
(`schema_repair_retry_recovers_from_bad_json`) without infinite loops
(`persistent_schema_failure_is_reported`).

## D9 — npm audit scoped to production for the CI gate
**Context:** `npm audit --audit-level=high` flags dev‑only tooling (vite/vitest/esbuild:
dev‑server SSRF and test‑mocker path traversal) that **never ships** in the Tauri binary
(which bundles the compiled Rust + static built assets; `node_modules` is not shipped).
**Decision:** the blocking gate is `npm audit --omit=dev --audit-level=high` (production
supply chain — currently **0** vulnerabilities). A non‑blocking `npm run audit:js:dev`
retains full visibility, and the dev advisories are tracked in SECURITY.md.
**Consequence:** CI fails on anything that reaches users, without a risky
vite 5→8 / vitest 2→5 toolchain migration at release time. Revisit when we migrate.

## D10 — Removed the unused `validator` crate
**Context:** IPC input is validated explicitly in `commands.rs`/`services/extraction.rs`;
`validator` was declared but unused and pulled a vulnerable `idna 0.5` (RUSTSEC‑2024‑0421).
**Decision:** remove the dependency.
**Consequence:** `cargo audit` is clean of error‑level advisories, and there is no dead
dependency.

## D11 — Severity never by colour alone
**Decision:** every severity/urgency indicator pairs a colour with an **icon and a word**
(`SeverityBadge.tsx`, `DeadlineCountdown.tsx`), and forced‑colors/high‑contrast is
supported.
**Consequence:** WCAG 2.2 (1.4.1 Use of Color) is met by construction.

## D12 — Legal aid from a verifiable pack only, with release gating
**Decision:** `legal_aid.json` entries carry `sourceUrl` + `verifiedOn`; entries with
`verifiedOn = null` are **hidden in release builds** and shipped as clearly marked TODO
placeholders. Olai never invents laws or section numbers.
**Consequence:** no fabricated legal references; the team fills and dates sources before
release.
