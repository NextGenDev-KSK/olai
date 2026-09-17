# Accessibility

Target: **WCAG 2.2 AA**. Accessibility is a first‑class requirement, tested automatically
and by manual assistive‑technology review.

## 1. Automated testing (axe)

- `vitest-axe` runs against rendered screens, scoped to
  `wcag2a, wcag2aa, wcag21a, wcag21aa` (`src/test/utils.tsx::checkA11y`).
- Covered with **zero violations**: the **Onboarding** screen, the **Dashboard** screen
  (which renders the help‑tier banner, Paper Card, deadline countdown, and the
  Overview / Conflicts / Actions / Ask / Missing‑Info tabs, plus the compare dialog and
  citation drawer through interaction), the **scam** dashboard variant, and the shared
  **component** suite (buttons, badges, pills, chips, dialogs, footer, cards).
- Run: `npm run test:coverage` (axe assertions run as part of the frontend suite).

## 2. Keyboard operation

- Full keyboard operability with a logical tab order and **visible focus rings**
  (`:focus-visible` styles in `src/index.css`).
- A **skip link** (`components/SkipLink.tsx`) jumps to main content.
- A **shortcuts dialog** opens with `?` (`components/ShortcutsDialog.tsx`).
- The **Compare** view supports `J`/`K` to move between numbered conflicts; the citation
  drawer and all dialogs (Radix UI) trap focus correctly and restore it on close, with no
  keyboard traps.
- The Playwright suite includes a keyboard‑navigation assertion in the compare flow.

## 3. Semantics & announcements

- Semantic landmarks (`header`, `main`, `nav`, `section`) and a single logical heading
  order; ARIA is used only where native semantics are insufficient.
- `aria-live` regions announce progress stages and results so screen‑reader users hear
  updates without moving focus.
- Radix UI primitives provide correct roles/labels for tabs, dialogs, radio groups, and
  tooltips.

## 4. Colour, contrast, and motion

- **Severity is never colour alone** — it is always paired with an **icon and a word**
  (`SeverityBadge.tsx`, `StatusPill.tsx`, `DeadlineCountdown.tsx`).
- Text/background pairs meet **≥ 4.5:1**; semantic colour tokens are defined per theme in
  `src/index.css`.
- **Windows High Contrast / forced‑colors** is supported (forced‑colors media handling and
  system colour keywords); **`prefers-reduced-motion`** is respected; **system
  dark/light** theme is followed and can be toggled.

## 5. Zoom, targets, and easy‑read

- Layout uses relative units and reflows to **200% zoom** without loss of content or
  function.
- Interactive targets are **≥ 44 × 44 px** (`min-h-touch` / `min-w-touch` utilities).
- An **easy‑read mode** simplifies presentation for lower literacy / cognitive load.

## 6. Language & fonts

- The document `lang` attribute is set per language (`en` / `ta` / `hi`), and content
  switches with the UI.
- **Noto Sans Tamil** and **Noto Sans Devanagari** are bundled locally (no remote fonts,
  consistent with the CSP) so Tamil and Hindi render correctly offline.
- A **“Listen”** button (Web Speech API, `components/ListenButton.tsx`) reads content
  aloud, with graceful fallback when no system voice is available.

## 7. Manual assistive‑technology testing

Automated tools catch a subset of issues; the following manual checklist is performed on
the Windows target with **Narrator** (and should be re‑run before release). Record pass/
fail and notes here:

| Scenario | Expected | Result |
| --- | --- | --- |
| Tab through Onboarding | All controls reachable, labelled, focus visible | _to record_ |
| Start demo; hear tier banner + deadline | Announced via `aria-live` | _to record_ |
| Navigate Dashboard tabs with arrow keys | Correct tab semantics announced | _to record_ |
| Open Compare; move with `J`/`K` | Conflict number and content announced | _to record_ |
| Open citation drawer | Quote, page, and verification status announced | _to record_ |
| Trigger “Listen” | Content is read aloud (or graceful message) | _to record_ |
| Switch to Tamil / Hindi | `lang` updates; Narrator uses correct pronunciation | _to record_ |
| “Delete everything” | Returns to Onboarding; announced | _to record_ |

> **Status:** automated axe checks pass with zero violations across the covered screens.
> The Narrator column is a manual pass to be completed on the target hardware and dated.
