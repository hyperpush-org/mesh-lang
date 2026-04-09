<script>
  import { marked } from 'marked';
  import { Copy, Check, Download, User, Tag, ArrowLeft } from 'lucide-svelte';
  export let data;

  let copied = false;
  async function copyInstall() {
    if (!data.pkg) return;
    await navigator.clipboard.writeText(`meshpkg install ${data.pkg.name}`);
    copied = true;
    setTimeout(() => { copied = false; }, 2000);
  }

  $: readmeHtml = data.pkg?.readme ? marked.parse(data.pkg.readme) : null;

  function formatBytes(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / 1048576).toFixed(1) + ' MB';
  }
</script>

<svelte:head>
  {#if data.pkg}
    <title>{data.pkg.name} — Mesh Packages</title>
    <meta name="description" content={data.pkg.description || `${data.pkg.name} — a Mesh package.`} />
    <meta property="og:title" content="{data.pkg.name} — Mesh Packages" />
    <meta property="og:description" content={data.pkg.description || `Install with meshpkg install ${data.pkg.name}`} />
    <meta property="og:url" content="https://packages.meshlang.dev/packages/{data.pkg.name}" />
    <link rel="canonical" href="https://packages.meshlang.dev/packages/{data.pkg.name}" />
  {:else if data.notFound}
    <title>Package not found — Mesh Packages</title>
    <meta name="robots" content="noindex" />
  {:else}
    <title>Package — Mesh Packages</title>
  {/if}
</svelte:head>

{#if data.notFound}
  <section class="py-24 text-center">
    <div class="mx-auto max-w-md">
      <h1 class="text-2xl font-bold text-foreground">Package not found</h1>
      <p class="mt-2 text-muted-foreground">This package doesn't exist or has been removed.</p>
      <a href="/" class="mt-6 inline-flex items-center gap-2 rounded-lg bg-foreground px-5 py-2.5 text-sm font-medium text-primary-foreground no-underline transition-opacity hover:opacity-90">
        <ArrowLeft class="size-3.5" />
        Browse all packages
      </a>
    </div>
  </section>
{:else if data.error}
  <section class="py-24 text-center">
    <p class="text-muted-foreground">{data.error}</p>
  </section>
{:else if data.pkg}

  <!-- Header banner -->
  <section class="border-b border-border/40 bg-gradient-to-b from-muted/30 to-background">
    <div class="mx-auto max-w-6xl px-4 py-10">
      <!-- Breadcrumb -->
      <a href="/" class="mb-4 inline-flex items-center gap-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground no-underline">
        <ArrowLeft class="size-3.5" />
        All packages
      </a>

      <div class="flex flex-wrap items-start justify-between gap-4">
        <div class="min-w-0 flex-1">
          <div class="text-xs font-mono uppercase tracking-widest text-muted-foreground mb-1">Package</div>
          <h1 class="text-3xl font-bold tracking-tight text-foreground break-all">{data.pkg.name}</h1>
          {#if data.pkg.description}
            <p class="mt-2 text-base text-muted-foreground">{data.pkg.description}</p>
          {/if}
        </div>
        {#if data.pkg.latest}
          <span class="shrink-0 rounded-lg bg-card border border-border px-3 py-1.5 font-mono text-sm text-muted-foreground">
            v{data.pkg.latest.version}
          </span>
        {/if}
      </div>

      <!-- Install command terminal block -->
      <div class="mt-6 flex items-center gap-3 rounded-lg border border-border bg-card px-5 py-4 max-w-xl">
        <span class="font-mono text-sm text-muted-foreground select-none shrink-0">$</span>
        <code class="flex-1 font-mono text-sm text-foreground truncate" title="meshpkg install {data.pkg.name}">meshpkg install {data.pkg.name}</code>
        <button
          on:click={copyInstall}
          class="shrink-0 rounded-md p-1.5 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
          aria-label="Copy install command"
        >
          {#if copied}
            <Check class="size-4 text-foreground" />
          {:else}
            <Copy class="size-4" />
          {/if}
        </button>
      </div>
    </div>
  </section>

  <!-- Two-column body: README + sidebar -->
  <section class="py-10">
    <div class="mx-auto max-w-6xl px-4">
      <div class="flex flex-col gap-8 lg:flex-row lg:gap-12">

        <!-- Main: README -->
        <div class="flex-1 min-w-0">
          {#if readmeHtml}
            <div class="rounded-xl border border-border bg-card p-8 overflow-hidden">
              <div class="prose prose-neutral max-w-none dark:prose-invert prose-pre:overflow-x-auto prose-code:break-all">
                {@html readmeHtml}
              </div>
            </div>
          {:else}
            <div class="rounded-xl border border-border bg-card p-12 text-center text-muted-foreground text-sm">
              No README provided.
            </div>
          {/if}
        </div>

        <!-- Sidebar: metadata -->
        <aside class="w-full lg:w-72 shrink-0 space-y-4">

          <!-- Metadata card -->
          <div class="rounded-xl border border-border bg-card p-5 space-y-3">
            <h2 class="text-sm font-semibold text-foreground">Package info</h2>
            <dl class="space-y-2.5 text-sm">
              {#if data.pkg.owner}
                <div class="flex items-center gap-2.5 text-muted-foreground">
                  <User class="size-3.5 shrink-0" />
                  <dt class="sr-only">Owner</dt>
                  <dd class="truncate">{data.pkg.owner}</dd>
                </div>
              {/if}
              {#if data.pkg.latest}
                <div class="flex items-center gap-2.5 text-muted-foreground">
                  <Tag class="size-3.5 shrink-0" />
                  <dt class="sr-only">Latest version</dt>
                  <dd class="font-mono">v{data.pkg.latest.version}</dd>
                </div>
              {/if}
              <div class="flex items-center gap-2.5 text-muted-foreground">
                <Download class="size-3.5 shrink-0" />
                <dt class="sr-only">Downloads</dt>
                <dd class="tabular-nums">{data.pkg.download_count.toLocaleString()} downloads</dd>
              </div>
            </dl>
          </div>

          <!-- Install card -->
          <div class="rounded-xl border border-border bg-card p-5">
            <h2 class="text-sm font-semibold text-foreground mb-3">Install</h2>
            <pre class="rounded-lg border border-border bg-muted px-3 py-2.5 font-mono text-xs text-foreground overflow-x-auto">meshpkg install {data.pkg.name}</pre>
          </div>

          <!-- Version history card -->
          {#if data.versions && data.versions.length > 0}
            <div class="rounded-xl border border-border bg-card p-5">
              <h2 class="text-sm font-semibold text-foreground mb-3">Versions</h2>
              <ul class="space-y-2">
                {#each data.versions as ver}
                  <li class="flex items-center justify-between gap-2 text-sm">
                    <span class="font-mono text-foreground">v{ver.version}</span>
                    <span class="text-muted-foreground text-xs tabular-nums">
                      {new Date(ver.published_at).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })}
                    </span>
                  </li>
                {/each}
              </ul>
            </div>
          {/if}

        </aside>
      </div>
    </div>
  </section>
{/if}
