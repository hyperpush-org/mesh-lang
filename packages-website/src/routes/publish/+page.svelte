<script>
  import { Github, ArrowRight } from 'lucide-svelte';

  const AUTH_URL = 'https://api.packages.meshlang.dev/auth/github';
</script>

<svelte:head>
  <title>Publish a Package — Mesh Packages</title>
  <meta name="description" content="Share your Mesh library with the community. Sign in with GitHub, get a publish token, and ship in under a minute." />
  <meta property="og:title" content="Publish a Package — Mesh Packages" />
  <meta property="og:description" content="Share your Mesh library with the community. Publishing takes under a minute." />
  <meta property="og:url" content="https://packages.meshlang.dev/publish" />
  <link rel="canonical" href="https://packages.meshlang.dev/publish" />
</svelte:head>

<!-- Hero -->
<section class="relative overflow-hidden border-b border-border/40">
  <div class="absolute inset-0 bg-gradient-to-b from-muted/50 via-background to-background"></div>
  <div class="relative mx-auto max-w-2xl px-4 py-16 sm:py-20 text-center">
    <div class="text-xs font-mono uppercase tracking-widest text-muted-foreground">Publishing</div>
    <h1 class="mt-3 text-4xl font-bold tracking-tight text-foreground sm:text-5xl">
      Publish a package
    </h1>
    <p class="mx-auto mt-4 max-w-lg text-lg text-muted-foreground">
      Share your Mesh library with the community. Publishing takes under a minute.
    </p>
    <a
      href={AUTH_URL}
      class="mt-8 inline-flex items-center gap-2 rounded-lg bg-foreground px-5 py-2.5 text-sm font-semibold text-primary-foreground shadow-sm transition-opacity hover:opacity-90 no-underline"
    >
      <Github class="size-4" />
      Sign in with GitHub
    </a>
    <p class="mt-3 text-xs text-muted-foreground">
      A publish token will be generated automatically after sign-in.
    </p>
  </div>
</section>

<!-- Steps -->
<section class="py-12 sm:py-16">
  <div class="mx-auto max-w-2xl px-4">
    <ol class="space-y-10">

      <li class="flex gap-4">
        <span class="flex size-9 shrink-0 items-center justify-center rounded-full border border-border bg-card font-mono text-sm text-muted-foreground shadow-sm">1</span>
        <div class="pt-1">
          <h2 class="text-base font-semibold text-foreground">Sign in with GitHub</h2>
          <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed">
            Click the button above. After authorizing, you'll be redirected back here with your publish token ready to copy.
          </p>
        </div>
      </li>

      <li class="flex gap-4">
        <span class="flex size-9 shrink-0 items-center justify-center rounded-full border border-border bg-card font-mono text-sm text-muted-foreground shadow-sm">2</span>
        <div class="pt-1">
          <h2 class="text-base font-semibold text-foreground">Save your token</h2>
          <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed mb-3">
            Run this command with the token shown after sign-in:
          </p>
          <pre class="rounded-lg border border-border bg-card px-4 py-3 font-mono text-sm text-foreground overflow-x-auto">meshpkg login --token &lt;your-token&gt;</pre>
        </div>
      </li>

      <li class="flex gap-4">
        <span class="flex size-9 shrink-0 items-center justify-center rounded-full border border-border bg-card font-mono text-sm text-muted-foreground shadow-sm">3</span>
        <div class="pt-1">
          <h2 class="text-base font-semibold text-foreground">Create a <code class="font-mono text-sm">mesh.toml</code></h2>
          <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed mb-3">
            Your package name must be scoped to your GitHub username:
          </p>
          <pre class="rounded-lg border border-border bg-card px-4 py-3 font-mono text-sm text-foreground overflow-x-auto">[package]
name = "your-login/your-package"
version = "1.0.0"
description = "What your package does"
license = "MIT"</pre>
        </div>
      </li>

      <li class="flex gap-4">
        <span class="flex size-9 shrink-0 items-center justify-center rounded-full border border-border bg-card font-mono text-sm text-muted-foreground shadow-sm">4</span>
        <div class="pt-1">
          <h2 class="text-base font-semibold text-foreground">Publish</h2>
          <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed mb-3">
            Run from your project directory:
          </p>
          <pre class="rounded-lg border border-border bg-card px-4 py-3 font-mono text-sm text-foreground overflow-x-auto">meshpkg publish</pre>
          <p class="mt-2 text-xs text-muted-foreground">
            meshpkg reads <code class="font-mono">mesh.toml</code> and uploads your package. Your files will be live within seconds.
          </p>
        </div>
      </li>

    </ol>
  </div>
</section>
