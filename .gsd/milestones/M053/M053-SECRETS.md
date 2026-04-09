# Secrets Manifest

**Milestone:** 
**Generated:** 

### FLY_API_TOKEN

**Service:** 
**Status:** collected
**Destination:** dotenv

1. Open the Fly personal access tokens page.
2. Create a new token scoped for deploy and app inspection.
3. Name it for the M053 deploy-proof workflow so later rotation is obvious.
4. Copy the token once when Fly shows it.
5. Store it as `FLY_API_TOKEN` for local verifier/deploy use and mirror it into the hosted secret store used by the deploy workflow.

### DATABASE_URL

**Service:** 
**Status:** skipped
**Destination:** dotenv

1. Open the dashboard for the PostgreSQL service that will back the serious starter proof.
2. Navigate to the connection or secrets section for that database/app.
3. Create or locate an application user/database with migration and CRUD permissions.
4. Copy the full connection string, including SSL/query parameters required by the provider.
5. Store it as `DATABASE_URL` for the starter deploy bundle, smoke rails, and clustered runtime proof.

### MESH_CLUSTER_COOKIE

**Service:** 
**Status:** skipped
**Destination:** dotenv

1. Open the deployed starter app in the Fly dashboard.
2. Navigate to the app secrets/settings area.
3. Generate a strong random shared secret for cluster membership.
4. Set the same secret on every node or deployment instance participating in the clustered starter proof.
5. Store it as `MESH_CLUSTER_COOKIE` for local replay and mirror it into hosted deploy secrets when the workflow needs it.
