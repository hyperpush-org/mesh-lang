const clientRoot = '../hyperpush-mono/mesher/client'
const devPort = 3000
const prodPort = 3001

function parseBaseUrl(name, value, expectedPort) {
  const parsedBaseUrl = new URL(value)

  if (!['http:', 'https:'].includes(parsedBaseUrl.protocol)) {
    throw new Error(`Invalid ${name} protocol: ${parsedBaseUrl.protocol}`)
  }

  if (!['', '/'].includes(parsedBaseUrl.pathname)) {
    throw new Error(`${name} must point at the app origin, received path ${parsedBaseUrl.pathname}`)
  }

  if (parsedBaseUrl.hostname !== '127.0.0.1' && parsedBaseUrl.hostname !== 'localhost') {
    throw new Error(`${name} must target localhost or 127.0.0.1, received host ${parsedBaseUrl.hostname}`)
  }

  if (parsedBaseUrl.port !== String(expectedPort)) {
    throw new Error(`${name} must target port ${expectedPort}, received ${parsedBaseUrl.port || '(default)'}`)
  }

  return parsedBaseUrl
}

function selectNamedItems(kind, items, requestedProjectName) {
  if (!requestedProjectName) {
    return items
  }

  const selectedItem = items.find((item) => item.name === requestedProjectName)

  if (!selectedItem) {
    throw new Error(
      `Unknown ${kind} project "${requestedProjectName}". Expected one of: ${items
        .map((item) => item.name)
        .join(', ')}`,
    )
  }

  return [selectedItem]
}

const requestedProjectName =
  process.env.PLAYWRIGHT_PROJECT?.trim() || process.env.npm_config_project?.trim() || null

const devBaseUrl = parseBaseUrl(
  'PLAYWRIGHT_BASE_URL',
  process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${devPort}`,
  devPort,
)
const prodBaseUrl = parseBaseUrl(
  'PLAYWRIGHT_PROD_BASE_URL',
  process.env.PLAYWRIGHT_PROD_BASE_URL ?? `http://127.0.0.1:${prodPort}`,
  prodPort,
)

const projects = [
  {
    name: 'dev',
    use: {
      baseURL: devBaseUrl.toString(),
    },
  },
  {
    name: 'prod',
    use: {
      baseURL: prodBaseUrl.toString(),
    },
  },
]

const webServers = [
  {
    name: 'dev',
    command: `env -u npm_config_project npm --prefix ${clientRoot} run dev -- --host 127.0.0.1 --port ${devPort}`,
    port: devPort,
    timeout: 30_000,
    reuseExistingServer: false,
  },
  {
    name: 'prod',
    command: `env -u npm_config_project npm --prefix ${clientRoot} run build && env -u npm_config_project PORT=${prodPort} npm --prefix ${clientRoot} run start`,
    port: prodPort,
    timeout: 60_000,
    reuseExistingServer: false,
  },
]

/** @type {import('@playwright/test').PlaywrightTestConfig} */
const config = {
  testDir: `${clientRoot}/tests/e2e`,
  timeout: 30_000,
  expect: {
    timeout: 10_000,
  },
  fullyParallel: false,
  retries: 0,
  reporter: [['list']],
  use: {
    browserName: 'chromium',
    trace: 'on-first-retry',
    video: 'retain-on-failure',
  },
  projects: selectNamedItems('Playwright', projects, requestedProjectName),
  webServer: selectNamedItems('web server', webServers, requestedProjectName).map(
    ({ name: _name, ...server }) => server,
  ),
}

export default config
