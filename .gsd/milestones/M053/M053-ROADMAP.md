# M053: 

## Vision
Make the generated Postgres Todo starter the truthful deployable clustered path, keep SQLite explicitly local-only, and pull packages-website verification into the normal hosted release/deploy evidence chain without making Fly the public product contract.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Generated Postgres starter owns staged deploy truth | high | — | ✅ | Generate a fresh Postgres Todo starter, stage a deploy bundle outside the source tree, run the staged artifact against PostgreSQL, exercise CRUD plus `meshc cluster` inspection against that running starter, and retain starter-owned evidence. |
| S02 | Generated Postgres starter proves clustered failover truth | high | S01 | ✅ | Run the generated Postgres starter in a production-like clustered replay, hit its real endpoints, inspect `meshc cluster status|continuity|diagnostics`, trigger the named node-loss or failover path, and confirm the starter survives with truthful artifacts. |
| S03 | Hosted evidence chain fails on starter deploy or packages drift | medium | S02 | ✅ | Run the normal hosted release/deploy chain and show it fails when the serious starter deploy proof breaks or when packages-website deploy/public-surface checks drift, with workflow evidence that makes packages part of the same public contract. |
| S04 | Public docs and Fly reference assets match the shipped contract | medium | S02, S03 | ✅ | Read the generated/example/public docs surfaces for the starters and packages story, then verify they present SQLite as local-only, Postgres as the serious deployable starter, and Fly as the current reference proof environment without replacing the portable contract. |
| S05 | Hosted workflow evidence closes the starter/packages contract | high | S03 | ✅ | Run `bash scripts/verify-m053-s03.sh` to green so `.tmp/m053-s03/verify/status.txt` becomes `ok`, `remote-runs.json` shows fresh successful `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs on the expected refs, and the hosted chain honestly carries starter failover plus packages/public-surface proof. |
| S06 | Hosted failover promotion truth and annotated tag reroll | high | S05 | ✅ | Run `bash scripts/verify-m053-s03.sh` to green so `.tmp/m053-s03/verify/status.txt` is `ok`, `remote-runs.json` shows fresh successful `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs on the expected refs, and `refs/tags/v0.1.0^{}` resolves after the annotated reroll. |
