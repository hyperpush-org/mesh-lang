---
estimated_steps: 4
estimated_files: 6
skills_used:
  - react-best-practices
---

# T01: Convert the mixed-route breakdown into structured Issues/Alerts/Settings tables

**Slice:** S02 — Mixed-surface audit
**Milestone:** M061

## Description

Replace the prose-only `## mixed-route breakdown` bullets in `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` with three structured markdown tables under `### Issues`, `### Alerts`, and `### Settings`. Each row should use a stable backticked surface key plus a level (`panel`, `subsection`, `tab`, or `control`), a normalized classification (`mixed`, `live`, `mock-only`, or `shell-only`), at least one backticked code anchor, at least one backticked proof suite, a live-seam summary, and a boundary note. Route-specific minimum row sets are: Issues = `overview`, `list`, `detail`, `live-actions`, `shell-controls`, `proof-harness`; Alerts = `overview`, `list`, `detail`, `live-actions`, `shell-controls`; Settings = `general`, `team`, `api-keys`, `alert-rules`, `alert-channels`, `bounty`, `token`, `integrations`, `billing`, `security`, `notifications`, `profile`. Keep runtime `fallback` described in row notes/diagnostics rather than as the durable row classification.

## Steps

1. Audit the current Issues, Alerts, and Settings component seams and select one stable surface key per row that the parser can lock later without inference.
2. Replace the prose-only mixed-route bullets with three markdown tables that capture level, normalized classification, code evidence, proof evidence, live-seam summary, and boundary note for each required row.
3. For Settings, distinguish General/Team/API Keys/Alert Rules as live or mixed surfaces, keep Alert Channels as shell-only, and keep every other tab explicitly mock-only.
4. Keep fallback semantics in notes and invariants instead of as a canonical classification so the document describes enduring support boundaries rather than transient bootstrap failures.

## Must-Haves

- [ ] Three markdown tables exist under `### Issues`, `### Alerts`, and `### Settings` with stable surface keys and levels.
- [ ] Every documented row has a normalized classification plus non-empty code evidence, proof evidence, live seam, and boundary note cells.
- [ ] The document honestly separates supported live actions, grouped shell-only controls, and diagnostic-only proof harness chrome.

## Verification

- `python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
for heading in ('### Issues', '### Alerts', '### Settings'):
    assert heading in text, heading
for needle in (
    '| `overview` | `panel` | `mixed` |',
    '| `live-actions` | `control` | `live` |',
    '| `shell-controls` | `control` | `shell-only` |',
    '| `proof-harness` | `control` | `shell-only` |',
    '| `general` | `panel` | `mixed` |',
    '| `team` | `panel` | `live` |',
    '| `alert-channels` | `subsection` | `shell-only` |',
    '| `profile` | `tab` | `mock-only` |',
):
    assert needle in text, needle
PY`

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — canonical top-level inventory from S01 that needs mixed-route expansion.
- `../hyperpush-mono/mesher/client/components/dashboard/issues-page.tsx` — source and proof-harness boundary for Issues.
- `../hyperpush-mono/mesher/client/components/dashboard/issue-detail.tsx` — supported issue actions and shell-only chrome anchors.
- `../hyperpush-mono/mesher/client/components/dashboard/alert-detail.tsx` — supported alert actions and shell-only control anchors.
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx` — tab support labels, mock-only banners, and section-level test ids.
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx` — Settings live-section/source truth for General, Team, API Keys, and Alert Rules.

## Expected Output

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — structured mixed-surface tables for Issues, Alerts, and Settings.
