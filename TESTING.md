# Testing

Testing strategy and coverage summary. The single command **`npm run verify`** runs the
whole gate: lint, typecheck, format check, all tests, coverage thresholds, dependency
audits, and a build. No phase is “done” while any check fails.

## Test pyramid

```
        e2e (Playwright, mocked IPC)         ← whole demo journey, keyboard, injection
      integration (cargo test, MockProvider) ← full pipeline on demo scenarios
   unit (cargo test / vitest + axe)          ← services, components, screens, a11y
```

## Rust (`npm run test:rust`)

- **Unit tests** (in‑module `#[cfg(test)]`): segmentation & span ids; citation
  verification (exact / OCR‑noisy / fabricated / over‑length / missing span); date engine
  (days, months, month‑end clamp, missing anchor, DD/MM assumption, overdue); PII masking
  (Aadhaar / PAN / phone / email / UPI / bank, overlap resolution); injection & hidden‑text
  scanner; safety tier overrides and verdict filter; file validation (magic bytes, caps,
  hashing); provider JSON parsing; error → code mapping; `ts-rs` binding export.
- **Integration tests** (`src-tauri/tests/pipeline_demo.rs`, via `MockProvider`, no
  network):
  - `eviction_demo_produces_three_conflicts_and_escalates`
  - `scam_demo_escalates_and_flags_injection`
  - `fabricated_citation_is_marked_not_confirmed`
  - `eviction_qa_is_grounded_in_documents`
  - `strategy_question_is_refused_without_a_model_call`
  - `reply_footnotes_are_verified`
  - `schema_repair_retry_recovers_from_bad_json`
  - `persistent_schema_failure_is_reported`
- **Result (latest run):** `113 passed` (unit, `app` feature) + `8 passed` (integration);
  `108 passed` with `--no-default-features` (pure logic). `0 failed`.
- **Coverage gate:** `npm run coverage:rust` → `cargo llvm-cov --fail-under-lines 90`.

## Frontend (`npm run test` / `npm run test:coverage`)

- **Vitest + React Testing Library** for every component and screen, including loading,
  error, and empty states; **vitest‑axe** accessibility assertions on the covered screens.
- **Result (latest run):** `11 files, 64 tests passed`.
- **Coverage (v8), enforced in `vitest.config.ts`:**

  | Metric | Threshold | Measured |
  | --- | --- | --- |
  | Lines | 90% | **98.18%** |
  | Statements | 90% | **98.18%** |
  | Functions | 90% | **93.51%** |
  | Branches | 85% | **89.96%** |

## End‑to‑end (`npm run test:e2e`)

Playwright drives the built app in Chromium against a **mocked Tauri IPC** layer
(`src/lib/mock-backend.ts`) — deterministic and offline.

- **Demo flow:** open → dashboard (tier + computed deadline) → conflicts → compare with
  `J`/`K` keyboard nav → grounded Ask → export `.ics` → delete everything.
- **Keyboard‑only flow** and **injection‑document flow** exercise keyboard reachability and
  the injection/hidden‑text surfacing.

## Benchmarks (`cargo bench`)

Criterion benchmarks for the two hottest paths: `segmentation` and citation
`verification` (`src-tauri/benches/`).

## Continuous integration

`.github/workflows/ci.yml` runs on **windows‑latest**: ESLint, `tsc`, Prettier check,
`cargo fmt --check`, `cargo clippy -D warnings`, frontend coverage, Rust coverage,
Playwright, `npm audit`, `cargo audit`, and a frontend build — then a separate job builds
the Tauri MSI/NSIS installer.

## How to reproduce locally

```bash
npm install
npm run verify        # the full gate
npm run test:e2e      # end-to-end (starts the dev server automatically)
```
