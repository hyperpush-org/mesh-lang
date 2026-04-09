export async function load({ fetch, params }) {
  try {
    // Fetch package metadata and versions list in parallel
    const [pkgRes, versionsRes] = await Promise.all([
      fetch(`https://api.packages.meshlang.dev/api/v1/packages/${params.name}`),
      fetch(`https://api.packages.meshlang.dev/api/v1/packages/${params.name}/versions`),
    ]);

    if (pkgRes.status === 404) return { pkg: null, versions: [], notFound: true };
    if (!pkgRes.ok) return { pkg: null, versions: [], error: 'Registry unavailable' };

    const pkg = await pkgRes.json();
    const versions = versionsRes.ok ? await versionsRes.json() : [];

    return { pkg, versions };
  } catch {
    return { pkg: null, versions: [], error: 'Failed to fetch package' };
  }
}
