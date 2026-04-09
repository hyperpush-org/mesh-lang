<script>
  import { Search, Package } from 'lucide-svelte';
  import { Download } from 'lucide-svelte';
  export let data;
</script>

<svelte:head>
  <title>{data.query ? `"${data.query}" — Search` : 'Search'} — Mesh Packages</title>
  <meta name="description" content={data.query ? `Search results for "${data.query}" on Mesh Packages.` : 'Search the Mesh package registry.'} />
  <meta name="robots" content="noindex" />
</svelte:head>

<section class="border-b border-border/40 bg-gradient-to-b from-muted/30 to-background">
  <div class="mx-auto max-w-6xl px-4 py-10">
    <div class="text-xs font-mono uppercase tracking-widest text-muted-foreground mb-1">Search</div>
    <h1 class="text-2xl font-bold tracking-tight text-foreground">
      {#if data.query}
        Results for <span class="text-muted-foreground">"{data.query}"</span>
      {:else}
        Search packages
      {/if}
    </h1>
    {#if !data.error && data.query}
      <p class="mt-1 text-sm text-muted-foreground tabular-nums">
        {data.packages.length} result{data.packages.length === 1 ? '' : 's'}
      </p>
    {/if}
  </div>
</section>

<section class="py-10">
  <div class="mx-auto max-w-6xl px-4">
    {#if data.error}
      <div class="rounded-xl border border-border bg-card p-8 text-center">
        <p class="text-muted-foreground">{data.error}</p>
      </div>
    {:else if !data.query}
      <div class="rounded-xl border border-border bg-card p-12 text-center">
        <div class="mx-auto flex h-12 w-12 items-center justify-center rounded-xl border border-border bg-muted">
          <Search class="size-5 text-muted-foreground" />
        </div>
        <p class="mt-4 text-muted-foreground">Enter a query to search packages.</p>
      </div>
    {:else if data.packages.length === 0}
      <div class="rounded-xl border border-border bg-card p-12 text-center">
        <div class="mx-auto flex h-12 w-12 items-center justify-center rounded-xl border border-border bg-muted">
          <Package class="size-5 text-muted-foreground" />
        </div>
        <p class="mt-4 text-muted-foreground">No packages found for "{data.query}".</p>
        <a href="/" class="mt-4 inline-block text-sm text-foreground underline underline-offset-4 hover:text-muted-foreground transition-colors">
          Browse all packages
        </a>
      </div>
    {:else}
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {#each data.packages as pkg}
          <a
            href="/packages/{pkg.name}"
            class="group block rounded-xl border border-border/60 bg-card p-6 transition-all duration-200 hover:border-foreground/20 hover:shadow-md no-underline"
          >
            <div class="flex items-start justify-between gap-3">
              <h3 class="text-sm font-semibold text-foreground leading-snug break-all min-w-0">
                {pkg.name}
              </h3>
              <span class="shrink-0 rounded-md bg-muted px-2 py-0.5 font-mono text-[11px] text-muted-foreground max-w-[140px] truncate" title="v{pkg.version}">
                v{pkg.version}
              </span>
            </div>
            <p class="mt-2.5 text-sm leading-relaxed text-muted-foreground line-clamp-2">
              {pkg.description || 'No description provided.'}
            </p>
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
