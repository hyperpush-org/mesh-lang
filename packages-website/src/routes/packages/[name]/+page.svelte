<script>
  // Redirect to the catch-all [...name] route
  export let data;
</script>

<svelte:head>
  {#if data.pkg}
    <title>{data.pkg.name} — Mesh Packages</title>
    <meta name="description" content={data.pkg.description || `${data.pkg.name} — a Mesh package.`} />
    <meta property="og:title" content="{data.pkg.name} — Mesh Packages" />
    <meta property="og:description" content={data.pkg.description || `Install with meshpkg install ${data.pkg.name}`} />
    <link rel="canonical" href="https://packages.meshlang.dev/packages/{data.pkg.name}" />
  {:else}
    <title>Package — Mesh Packages</title>
    <meta name="robots" content="noindex" />
  {/if}
</svelte:head>

<!-- This route handles single-segment names; the [...name] route handles scoped names. -->
{#if data.notFound}
  <section class="py-24 text-center">
    <div class="mx-auto max-w-md">
      <h1 class="text-2xl font-bold text-foreground">Package not found</h1>
      <p class="mt-2 text-muted-foreground">This package doesn't exist or has been removed.</p>
      <a href="/" class="mt-6 inline-block rounded-lg bg-foreground px-5 py-2.5 text-sm font-medium text-primary-foreground no-underline transition-opacity hover:opacity-90">
        Browse all packages
      </a>
    </div>
  </section>
{:else if data.error}
  <section class="py-24 text-center">
    <p class="text-muted-foreground">{data.error}</p>
  </section>
{:else if data.pkg}
  <!-- Render same as [...name] — redirect in practice -->
  <section class="py-24 text-center">
    <p class="text-muted-foreground">Redirecting…</p>
  </section>
{/if}
