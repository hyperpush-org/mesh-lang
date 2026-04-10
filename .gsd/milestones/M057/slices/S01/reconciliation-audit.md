# M057 S01 Reconciliation Audit

- Version: `m057-s01-reconciliation-ledger-v1`
- Inventory captured_at: `2026-04-10T06:20:21Z`
- Generated at: `2026-04-10T06:21:02Z`
- Ledger rows: `68`
- Project-backed rows: `63`
- Non-project rows: `5`
- Derived gaps: `1`

## Rollup

| Bucket | Count |
| --- | --- |
| `shipped-but-open` | `13` |
| `rewrite/split` | `21` |
| `keep-open` | `33` |
| `misfiled` | `1` |
| `missing-coverage` | `1` |
| `naming-drift` | `14` |

## shipped-but-open

| Issue | Project item | Repo action | Project action | Evidence refs |
| --- | --- | --- | --- | --- |
| `hyperpush#3` | `_none_` | `close_as_shipped` | `leave_untracked` | `14` |
| `hyperpush#4` | `_none_` | `close_as_shipped` | `leave_untracked` | `14` |
| `hyperpush#5` | `_none_` | `close_as_shipped` | `leave_untracked` | `14` |
| `mesh-lang#3` | `PVTI_lADOEExRVs4BUM59zgpjP54` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#4` | `PVTI_lADOEExRVs4BUM59zgpjP6M` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#5` | `PVTI_lADOEExRVs4BUM59zgpjP6o` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#6` | `PVTI_lADOEExRVs4BUM59zgpjP7A` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#8` | `PVTI_lADOEExRVs4BUM59zgpjTcY` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#9` | `PVTI_lADOEExRVs4BUM59zgpjTdQ` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#10` | `PVTI_lADOEExRVs4BUM59zgpjTdc` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#11` | `PVTI_lADOEExRVs4BUM59zgpjTd8` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#13` | `PVTI_lADOEExRVs4BUM59zgpjTe0` | `close_as_shipped` | `remove_from_project` | `10` |
| `mesh-lang#14` | `PVTI_lADOEExRVs4BUM59zgpjTfM` | `close_as_shipped` | `remove_from_project` | `10` |

## rewrite/split

| Issue | Project item | Repo action | Project action | Evidence refs |
| --- | --- | --- | --- | --- |
| `hyperpush#24` | `PVTI_lADOEExRVs4BUM59zgpjQAg` | `rewrite_scope` | `update_project_item` | `12` |
| `hyperpush#29` | `PVTI_lADOEExRVs4BUM59zgpjTg8` | `rewrite_scope` | `update_project_item` | `12` |
| `hyperpush#30` | `PVTI_lADOEExRVs4BUM59zgpjThc` | `rewrite_scope` | `update_project_item` | `12` |
| `hyperpush#31` | `PVTI_lADOEExRVs4BUM59zgpjThw` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#32` | `PVTI_lADOEExRVs4BUM59zgpjTiQ` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#35` | `PVTI_lADOEExRVs4BUM59zgpjTj8` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#36` | `PVTI_lADOEExRVs4BUM59zgpjTkg` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#37` | `PVTI_lADOEExRVs4BUM59zgpjTkw` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#38` | `PVTI_lADOEExRVs4BUM59zgpjTlU` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#39` | `PVTI_lADOEExRVs4BUM59zgpjTl0` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#40` | `PVTI_lADOEExRVs4BUM59zgpjTmo` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#41` | `PVTI_lADOEExRVs4BUM59zgpjTnA` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#42` | `PVTI_lADOEExRVs4BUM59zgpjTnc` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#43` | `PVTI_lADOEExRVs4BUM59zgpjTns` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#44` | `PVTI_lADOEExRVs4BUM59zgpjToU` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#45` | `PVTI_lADOEExRVs4BUM59zgpjTo8` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#46` | `PVTI_lADOEExRVs4BUM59zgpjTpg` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#47` | `PVTI_lADOEExRVs4BUM59zgpjTqE` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#48` | `PVTI_lADOEExRVs4BUM59zgpjTsc` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#49` | `PVTI_lADOEExRVs4BUM59zgpjTuA` | `rewrite_scope` | `update_project_item` | `9` |
| `hyperpush#50` | `PVTI_lADOEExRVs4BUM59zgpjTvk` | `rewrite_scope` | `update_project_item` | `9` |

## keep-open

| Issue | Project item | Repo action | Project action | Evidence refs |
| --- | --- | --- | --- | --- |
| `hyperpush#2` | `_none_` | `keep_open` | `leave_untracked` | `8` |
| `hyperpush#11` | `PVTI_lADOEExRVs4BUM59zgpjP7c` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#12` | `PVTI_lADOEExRVs4BUM59zgpjP70` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#13` | `PVTI_lADOEExRVs4BUM59zgpjP8E` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#14` | `PVTI_lADOEExRVs4BUM59zgpjP8c` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#15` | `PVTI_lADOEExRVs4BUM59zgpjP84` | `keep_open` | `update_project_item` | `15` |
| `hyperpush#16` | `PVTI_lADOEExRVs4BUM59zgpjP9Q` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#17` | `PVTI_lADOEExRVs4BUM59zgpjP9k` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#18` | `PVTI_lADOEExRVs4BUM59zgpjP-I` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#19` | `PVTI_lADOEExRVs4BUM59zgpjP-g` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#20` | `PVTI_lADOEExRVs4BUM59zgpjP-8` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#21` | `PVTI_lADOEExRVs4BUM59zgpjP_Y` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#22` | `PVTI_lADOEExRVs4BUM59zgpjP_o` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#23` | `PVTI_lADOEExRVs4BUM59zgpjP_4` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#25` | `PVTI_lADOEExRVs4BUM59zgpjTfU` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#26` | `PVTI_lADOEExRVs4BUM59zgpjTgI` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#27` | `PVTI_lADOEExRVs4BUM59zgpjTgQ` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#28` | `PVTI_lADOEExRVs4BUM59zgpjTgw` | `keep_open` | `keep_in_project` | `9` |
| `hyperpush#33` | `PVTI_lADOEExRVs4BUM59zgpjTjE` | `keep_open` | `update_project_item` | `15` |
| `hyperpush#34` | `PVTI_lADOEExRVs4BUM59zgpjTj0` | `keep_open` | `update_project_item` | `15` |
| `hyperpush#51` | `PVTI_lADOEExRVs4BUM59zgpjdNY` | `keep_open` | `update_project_item` | `15` |
| `hyperpush#52` | `PVTI_lADOEExRVs4BUM59zgpjdOc` | `keep_open` | `update_project_item` | `15` |
| `hyperpush#53` | `PVTI_lADOEExRVs4BUM59zgpjdPM` | `keep_open` | `update_project_item` | `15` |
| `hyperpush#54` | `PVTI_lADOEExRVs4BUM59zgpjg5Q` | `keep_open` | `update_project_item` | `12` |
| `hyperpush#55` | `PVTI_lADOEExRVs4BUM59zgpjg5w` | `keep_open` | `update_project_item` | `12` |
| `hyperpush#56` | `PVTI_lADOEExRVs4BUM59zgpjg6w` | `keep_open` | `update_project_item` | `12` |
| `hyperpush#57` | `PVTI_lADOEExRVs4BUM59zgpjjYw` | `keep_open` | `update_project_item` | `15` |
| `mesh-lang#7` | `PVTI_lADOEExRVs4BUM59zgpjTbQ` | `keep_open` | `keep_in_project` | `4` |
| `mesh-lang#12` | `PVTI_lADOEExRVs4BUM59zgpjTec` | `keep_open` | `keep_in_project` | `4` |
| `mesh-lang#15` | `PVTI_lADOEExRVs4BUM59zgpjaRw` | `keep_open` | `keep_in_project` | `4` |
| `mesh-lang#16` | `PVTI_lADOEExRVs4BUM59zgpjaR8` | `keep_open` | `keep_in_project` | `4` |
| `mesh-lang#17` | `PVTI_lADOEExRVs4BUM59zgpjaSA` | `keep_open` | `keep_in_project` | `4` |
| `mesh-lang#18` | `PVTI_lADOEExRVs4BUM59zgpjaSE` | `keep_open` | `keep_in_project` | `4` |

## misfiled

| Issue | Project item | Repo action | Project action | Evidence refs |
| --- | --- | --- | --- | --- |
| `hyperpush#8` | `_none_` | `move_to_mesh_lang` | `create_project_item` | `14` |

## missing-coverage

| Gap | Repo action | Project action | Evidence refs |
| --- | --- | --- | --- |
| `/pitch` | `create_missing_issue` | `create_project_item` | `7` |

## naming-drift

| Issue | Primary bucket | Repo truth | Workspace truth |
| --- | --- | --- | --- |
| `hyperpush#8` | `misfiled` | `hyperpush-org/mesh-lang` | `website/docs/.vitepress/* tracked directly in mesh-lang` |
| `hyperpush#15` | `keep-open` | `hyperpush-org/hyperpush` | `mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo` |
| `hyperpush#24` | `rewrite-split` | `hyperpush-org/hyperpush` | `local compatibility path mesher -> ../hyperpush-mono/mesher; workspace helper canonicalizes both hyperpush and hyperpush-mono to workspace root hyperpush-mono` |
| `hyperpush#29` | `rewrite-split` | `hyperpush-org/hyperpush` | `local compatibility path mesher -> ../hyperpush-mono/mesher; workspace helper canonicalizes both hyperpush and hyperpush-mono to workspace root hyperpush-mono` |
| `hyperpush#30` | `rewrite-split` | `hyperpush-org/hyperpush` | `local compatibility path mesher -> ../hyperpush-mono/mesher; workspace helper canonicalizes both hyperpush and hyperpush-mono to workspace root hyperpush-mono` |
| `hyperpush#33` | `keep-open` | `hyperpush-org/hyperpush` | `mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo` |
| `hyperpush#34` | `keep-open` | `hyperpush-org/hyperpush` | `mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo` |
| `hyperpush#51` | `keep-open` | `hyperpush-org/hyperpush` | `mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo` |
| `hyperpush#52` | `keep-open` | `hyperpush-org/hyperpush` | `mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo` |
| `hyperpush#53` | `keep-open` | `hyperpush-org/hyperpush` | `mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo` |
| `hyperpush#54` | `keep-open` | `hyperpush-org/hyperpush` | `local compatibility path mesher -> ../hyperpush-mono/mesher; workspace helper canonicalizes both hyperpush and hyperpush-mono to workspace root hyperpush-mono` |
| `hyperpush#55` | `keep-open` | `hyperpush-org/hyperpush` | `local compatibility path mesher -> ../hyperpush-mono/mesher; workspace helper canonicalizes both hyperpush and hyperpush-mono to workspace root hyperpush-mono` |
| `hyperpush#56` | `keep-open` | `hyperpush-org/hyperpush` | `local compatibility path mesher -> ../hyperpush-mono/mesher; workspace helper canonicalizes both hyperpush and hyperpush-mono to workspace root hyperpush-mono` |
| `hyperpush#57` | `keep-open` | `hyperpush-org/hyperpush` | `mesher/frontend-exp/lib/mock-data.ts via the compatibility symlink into the sibling product repo` |

## Derived gap evidence

### /pitch

The evaluator-facing /pitch route already shipped in the sibling product repo during M056, but there is still no dedicated repo issue or project row that records that delivered surface.

- proposed_repo_action: Create or repoint one hyperpush tracker row so the shipped /pitch route is represented explicitly instead of being implied only by milestone history.
- proposed_project_action: Create a project #1 item for the replacement /pitch tracking issue so the shipped surface is visible on the org board.
- evidence_refs:
  - `naming-ownership-map.json/surfaces/2` — Naming/ownership surface product_pitch_route used for canonical repo truth.
  - `mesher/landing/app/pitch/page.tsx:4` — The route file exists and renders the shipped PitchDeck surface.
  - `.gsd/milestones/M056/M056-SUMMARY.md:42` — M056 milestone summary records /pitch as shipped in the product repo.
  - `reconciliation-evidence.json/entries/3` — Matched reconciliation evidence entry pitch_route_missing_tracker_coverage.
  - `.gsd/milestones/M056/M056-SUMMARY.md:36` — M056 summary records /pitch as shipped.
  - `mesher/landing/app/pitch/page.tsx:4` — The shipped /pitch route still exists in the product surface.
  - `.gsd/PROJECT.md:20` — Project contract says /pitch lives in the sibling product repo after the split.
