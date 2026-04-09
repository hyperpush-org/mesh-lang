# Secrets Manifest

**Milestone:** M054
**Generated:** 2026-04-05

### FLY_API_TOKEN

**Service:** Fly.io
**Dashboard:** https://fly.io/dashboard/personal_access_tokens
**Format hint:** Fly personal access token (opaque token string)
**Status:** pending
**Destination:** dotenv

1. Sign in to Fly.io and open the Personal Access Tokens page.
2. Create a token with the scope needed to deploy, inspect, and verify the serious starter app.
3. Copy the token immediately and store it as `FLY_API_TOKEN` for local verifier and deploy commands.

### GH_TOKEN

**Service:** GitHub
**Dashboard:** https://github.com/settings/personal-access-tokens/new
**Format hint:** `github_pat_...` or another GitHub token format accepted by `gh`
**Status:** pending
**Destination:** dotenv

1. Open GitHub Personal Access Token creation for the account that can read workflow runs and, if later approved, perform any required rollout actions.
2. Create a fine-grained token with the minimum repository and Actions permissions needed for hosted proof inspection in this repo.
3. Copy the token and store it as `GH_TOKEN` so local verifier scripts and `gh` queries can authenticate non-interactively.
