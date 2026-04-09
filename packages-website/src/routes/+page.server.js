export async function load({ fetch }) {
  try {
    const res = await fetch('https://api.packages.meshlang.dev/api/v1/packages');
    if (!res.ok) return { packages: [], error: 'Registry unavailable' };
    const packages = await res.json();
    return { packages };
  } catch {
    return { packages: [], error: 'Failed to fetch packages' };
  }
}
