---
estimated_steps: 1
estimated_files: 5
skills_used: []
---

# T02: Stand up TanStack Start plumbing in place

Replace the framework plumbing in place: create the TanStack Start root route/router entry, move global CSS import to the new root, update package/config scripts and dependencies, and preserve aliases/components/mock-data imports so the current dashboard shell can boot under the new framework.

## Inputs

- `T01 findings`
- `Official TanStack Start Next.js migration guidance`
- `Existing dashboard components and mock-data modules`

## Expected Output

- `TanStack Start/Vite project plumbing in `../hyperpush-mono/mesher/frontend-exp/``
- `A bootable root route and router entry replacing the old Next root`

## Verification

Run the in-place app’s install/build/dev smoke and confirm the visible shell still mounts with the preserved command contract.
