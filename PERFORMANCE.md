# Performance

Targets and measured results. Where a metric depends on the host machine, the
measurement method is documented so it can be reproduced on target hardware.

## Targets

| Metric | Target |
| --- | --- |
| Installer size | ≲ 15 MB |
| Idle memory | ≲ 150 MB |
| Cold start | < 2 s on a typical laptop |

## Measured results

Measured on the development machine (Windows 11, `cargo build --release`, LTO + `opt-level =
"s"` + `strip` + `panic = "abort"`).

| Artifact | Size |
| --- | --- |
| **NSIS installer** `Olai_0.1.0_x64-setup.exe` | **2.22 MB** |
| **MSI installer** `Olai_0.1.0_x64_en-US.msi` | **3.31 MB** |
| Release binary `olai.exe` | **5.90 MB** |
| Frontend bundle (`dist/`, total) | **293 KB** raw (272 KB JS / **87 KB gzip** + 15 KB CSS) |

- **Installer size:** both installers are **well under** the 15 MB target — WebView2 is
  provided by the OS on Windows 11 and is **not** bundled. (Reproduce with
  `npm run tauri build`; artifacts land in `src-tauri/target/release/bundle/`.) The
  installer is smaller than the raw binary because the bundler compresses it.

### Memory

Sampling Olai’s own process tree (the Rust host + its WebView2 children only, isolated by
parent‑PID) a few seconds after launch:

| Measure | Value | Notes |
| --- | --- | --- |
| Rust host process (private) | **~23 MB** | The backend’s own footprint |
| Full tree, working set | ~370 MB | **Over‑counts** WebView2’s shared runtime pages, which are shared across all WebView2 apps system‑wide and are not incremental to Olai |

> **Caveat & method:** “working set” double‑counts shared WebView2 DLL pages, so it
> overstates the app’s real cost. The meaningful figure is **private working set**, which
> should be captured on a **clean machine with no other WebView2/Edge processes running**
> (the dev machine had 19 unrelated `msedgewebview2.exe` processes). To measure:
> launch `olai.exe`, then in PowerShell sum `Get-Counter '\Process(*)\Working Set - Private'`
> for `olai` and the `msedgewebview2` children whose parent chain is `olai.exe`.
> The Rust backend is lean (~23 MB); the remainder is the shared WebView2 host.

### Cold start

Cold start is dominated by WebView2 initialization; the Rust binary and 293 KB frontend
load quickly. To measure precisely, instrument from process start to the app’s
`window.__TAURI__` ready / first paint. On the dev machine the onboarding screen is
interactive within roughly a second of launch. **Record the measured value on target
hardware here** using the browser `performance` timeline or a wall‑clock harness.

## Efficiency design (what keeps it fast)

- **PDF parsing off the UI thread** — `pdfjs-dist` runs in a **Web Worker**
  (`src/lib/pdf.worker*`), lazy‑chunked away from the initial bundle
  (`vite.config.ts` `manualChunks`).
- **Async Rust services (tokio);** independent model calls run **concurrently**
  (`join!`) in the analysis pipeline.
- **Send only what’s needed** — the pipeline passes only relevant spans to the model, and
  uses a **cheaper model tier** for simple steps (`providers::ModelTier::Fast`).
- **Caching** — documents are keyed by **SHA‑256 hash** (and language) so repeated work is
  avoided.
- **Streaming progress** — the progress screen shows named stages via Tauri events.
- **Small release profile** — `lto = true`, `codegen-units = 1`, `opt-level = "s"`,
  `strip = true`, `panic = "abort"`.
- **Benchmarks** — Criterion benches for `segmentation` and citation `verification`
  (`src-tauri/benches/`), the two hottest CPU paths; run with `cargo bench`.

## How to reproduce

```bash
npm run build                      # frontend bundle sizes in dist/
cargo build --release --manifest-path src-tauri/Cargo.toml   # binary size
npm run tauri build                # full installer (size in bundle/)
cargo bench --manifest-path src-tauri/Cargo.toml             # micro-benchmarks
```
