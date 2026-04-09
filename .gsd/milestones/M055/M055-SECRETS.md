# Secrets Manifest

**Milestone:** M055
**Generated:** 2026-04-05

### GH_TOKEN

**Service:** GitHub
**Dashboard:** https://github.com/settings/personal-access-tokens
**Format hint:** `github_pat_...` or `ghp_...`
**Status:** pending
**Destination:** dotenv

1. Sign in to GitHub and open the personal access token settings page.
2. Create a token that can access the `mesh-lang` and `hyperpush-mono` repos and the workflow/run metadata needed for repo creation, ref updates, and hosted-evidence inspection.
3. Copy the token once and store it in the local dotenv used for repo-split verification and rollout commands.

### FLY_API_TOKEN

**Service:** Fly.io
**Dashboard:** https://fly.io/dashboard/personal_access_tokens
**Format hint:** long opaque token
**Status:** pending
**Destination:** dotenv

1. Sign in to Fly.io and open the personal access token page.
2. Create a token that can inspect and verify the apps that will stay language-owned in `mesh-lang` and any product apps that move under `hyperpush-mono`.
3. Copy the token once and store it in the local dotenv used for deploy verification or live Fly inspection during the split.
