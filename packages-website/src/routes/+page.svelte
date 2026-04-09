<script>
  import { onMount } from 'svelte';
  import { Download, Package, ArrowRight, Search } from 'lucide-svelte';
  export let data;

  let cards = [];
  onMount(() => {
    const observer = new IntersectionObserver(
      (entries) => entries.forEach(e => { if (e.isIntersecting) e.target.classList.add('is-visible'); }),
      { threshold: 0.08 }
    );
    cards.forEach(el => { if (el) observer.observe(el); });
    return () => observer.disconnect();
  });
</script>

<svelte:head>
  <title>Mesh Packages — Community package registry for the Mesh programming language</title>
  <meta name="description" content="Browse, search, and install community packages for the Mesh programming language. Publish your own with meshpkg." />
  <meta property="og:title" content="Mesh Packages" />
  <meta property="og:description" content="Community package registry for the Mesh programming language." />
  <meta property="og:url" content="https://packages.meshlang.dev" />
  <link rel="canonical" href="https://packages.meshlang.dev" />
</svelte:head>

<!-- Hero section -->
<section class="relative overflow-hidden border-b border-border/40">
  <!-- Subtle gradient background -->
  <div class="absolute inset-0 bg-gradient-to-b from-muted/50 via-background to-background"></div>
  <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-foreground/[0.03] via-transparent to-transparent"></div>

  <div class="relative mx-auto max-w-6xl px-4 py-20 sm:py-28 text-center">
    <h1 class="text-4xl font-bold tracking-tight text-foreground sm:text-5xl lg:text-6xl">
      Mesh Packages
    </h1>
    <p class="mx-auto mt-4 max-w-md text-lg text-muted-foreground sm:text-xl">
      Discover and share packages for the Mesh programming language.
    </p>

    {#if !data.error && data.packages.length > 0}
      <p class="mt-3 text-sm text-muted-foreground font-mono tabular-nums">
        {data.packages.length} package{data.packages.length === 1 ? '' : 's'} published
      </p>
    {/if}

    <!-- Hero search -->
    <form action="/search" method="GET" class="mt-8 mx-auto max-w-lg">
      <div class="relative">
        <Search class="absolute left-4 top-1/2 -translate-y-1/2 size-4 text-muted-foreground pointer-events-none" />
        <input
          name="q"
          placeholder="Search packages…"
          class="h-12 w-full rounded-xl border border-border bg-card pl-11 pr-28 text-sm text-foreground placeholder:text-muted-foreground/70 shadow-sm focus:outline-none focus:ring-2 focus:ring-foreground/10 focus:border-foreground/20 transition-all"
        />
        <button type="submit" class="absolute right-1.5 top-1/2 -translate-y-1/2 h-9 rounded-lg bg-foreground px-4 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90">
          Search
        </button>
      </div>
    </form>

    <!-- Quick links -->
    <div class="mt-6 flex items-center justify-center gap-4 text-sm">
      <a href="/publish" class="inline-flex items-center gap-1.5 text-muted-foreground transition-colors hover:text-foreground no-underline">
        Publish a package
        <ArrowRight class="size-3.5" />
      </a>
      <span class="text-border">·</span>
      <a href="https://meshlang.dev/docs/tooling" target="_blank" rel="noopener" class="inline-flex items-center gap-1.5 text-muted-foreground transition-colors hover:text-foreground no-underline">
        meshpkg docs
        <ArrowRight class="size-3.5" />
      </a>
    </div>
  </div>
</section>

<!-- Package grid section -->
<section class="py-12 sm:py-16">
  <div class="mx-auto max-w-6xl px-4">
    {#if data.error}
      <div class="rounded-xl border border-border bg-card p-8 text-center">
        <p class="text-muted-foreground">{data.error}</p>
      </div>
    {:else if data.packages.length === 0}
      <div class="rounded-xl border border-border bg-card p-16 text-center">
        <div class="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl border border-border bg-muted">
          <Package class="size-6 text-muted-foreground" />
        </div>
        <h2 class="mt-4 text-lg font-semibold text-foreground">No packages yet</h2>
        <p class="mt-2 text-sm text-muted-foreground">Be the first to publish a Mesh package.</p>
        <a href="https://meshlang.dev/docs/tooling" class="mt-6 inline-flex items-center gap-2 rounded-lg bg-foreground px-5 py-2.5 text-sm font-medium text-primary-foreground no-underline transition-opacity hover:opacity-90">
          Learn meshpkg
          <ArrowRight class="size-3.5" />
        </a>
      </div>
    {:else}
      <!-- Section header -->
      <div class="mb-6 flex items-center justify-between">
        <h2 class="text-lg font-semibold text-foreground">All packages</h2>
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {#each data.packages as pkg, i}
          <a
            href="/packages/{pkg.name}"
            bind:this={cards[i]}
            class="reveal reveal-delay-{Math.min(i % 3 + 1, 4)} group block rounded-xl border border-border/60 bg-card p-6 transition-all duration-200 hover:border-foreground/20 hover:shadow-md no-underline"
          >
            <!-- Package name + version -->
            <div class="flex items-start justify-between gap-3">
              <h3 class="text-sm font-semibold text-foreground leading-snug break-all min-w-0">
                {pkg.name}
              </h3>
              <span class="shrink-0 rounded-md bg-muted px-2 py-0.5 font-mono text-[11px] text-muted-foreground max-w-[140px] truncate" title="v{pkg.version}">
                v{pkg.version}
              </span>
            </div>

            <!-- Description -->
            <p class="mt-2.5 text-sm leading-relaxed text-muted-foreground line-clamp-2">
              {pkg.description || 'No description provided.'}
            </p>

            <!-- Footer hints -->
            <div class="mt-4 flex items-center gap-3 text-xs text-muted-foreground/70">
              {#if pkg.owner}
                <span class="truncate max-w-[120px]">{pkg.owner}</span>
              {/if}
              {#if pkg.download_count != null}
                <span class="flex items-center gap-1">
                  <Download class="size-3" />
                  {pkg.download_count.toLocaleString()}
                </span>
              {/if}
            </div>
          </a>
        {/each}
      </div>
    {/if}
  </div>
</section>
