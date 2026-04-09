# S01 Replan

**Milestone:** M044
**Slice:** S01
**Blocker Task:** T03
**Created:** 2026-03-29T18:22:23.276Z

## Blocker Description

T03 confirmed the typed Continuity builtin seam from T02 never actually landed: compiler typeck/builtins, MIR/codegen intrinsics, runtime exports, the public e2e contract, and `cluster-proof/work_continuity.mpl` still expose `String ! String` plus JSON decode shims, so the planned consumer rewrite and final verifier order is invalid.

## What Changed

Replaced the old closeout-only T04 with the missing typed-continuity recovery work, then inserted the delayed `cluster-proof` consumer rewrite and moved the fail-closed verifier to the end. Threat surface and requirement coverage are unchanged: the slice still delivers the same manifest-backed clustered boundary and typed public continuity contract, but now the remaining tasks are ordered so the public API exists before the proof app and verifier depend on it.
