const frontendExpRoot = '../hyperpush-mono/mesher/frontend-exp'
const devPort = 3000
const prodPort = 3001

/** @type {import('@playwright/test').PlaywrightTestConfig} */
const config = {
  testDir: `${frontendExpRoot}/tests/e2e`,
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
    baseURL: `http://127.0.0.1:${devPort}`,
  },
  projects: [
    {
      name: 'dev',
      use: {
        baseURL: `http://127.0.0.1:${devPort}`,
      },
    },
    {
      name: 'prod',
      use: {
        baseURL: `http://127.0.0.1:${prodPort}`,
      },
    },
  ],
  webServer: [
    {
      command: `npm --prefix ${frontendExpRoot} run dev -- --host 127.0.0.1 --port ${devPort}`,
      port: devPort,
      timeout: 30_000,
      reuseExistingServer: false,
    },
    {
      command: `npm --prefix ${frontendExpRoot} run build && PORT=${prodPort} npm --prefix ${frontendExpRoot} run start`,
      port: prodPort,
      timeout: 60_000,
      reuseExistingServer: false,
    },
  ],
}

export default config
