# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: mesher/landing/tests/pitch-route.spec.ts >> /pitch export >> export button uses browser print once per request from first, middle, and deep-linked slides
- Location: mesher/landing/tests/pitch-route.spec.ts:132:7

# Error details

```
Error: page.goto: Protocol error (Page.navigate): Cannot navigate to invalid URL
Call log:
  - navigating to "/pitch", waiting until "load"

```

# Test source

```ts
  49  | 
  50  |     await page.keyboard.press('ArrowRight')
  51  |     await expect(page).toHaveURL(/\/pitch#burst-load$/)
  52  |     await expect(page.getByTestId('pitch-current-marker')).toHaveText('Current frame · 02 / 06')
  53  |     await expect(page.getByTestId('pitch-route-shell')).toHaveAttribute(
  54  |       'data-active-slide-id',
  55  |       'burst-load',
  56  |     )
  57  |     await expect(page.getByTestId('pitch-agenda-item').nth(1)).toHaveAttribute('aria-current', 'step')
  58  | 
  59  |     for (let index = 0; index < 8; index += 1) {
  60  |       await page.keyboard.press('ArrowRight')
  61  |     }
  62  | 
  63  |     await expect(page).toHaveURL(/\/pitch#platform$/)
  64  |     await expect(page.getByTestId('pitch-current-marker')).toHaveText('Current frame · 06 / 06')
  65  |     await expect(page.getByTestId('pitch-next')).toBeDisabled()
  66  |   })
  67  | 
  68  |   test('navigation wheel bursts advance one frame at a time', async ({ page }) => {
  69  |     await page.goto('/pitch')
  70  |     await page.getByTestId('pitch-route-shell').hover()
  71  | 
  72  |     await page.mouse.wheel(0, 920)
  73  |     await page.mouse.wheel(0, 920)
  74  |     await page.mouse.wheel(0, 920)
  75  | 
  76  |     await expect(page).toHaveURL(/\/pitch#burst-load$/)
  77  |     await expect(page.getByTestId('pitch-current-marker')).toHaveText('Current frame · 02 / 06')
  78  | 
  79  |     await page.waitForTimeout(650)
  80  |     await page.mouse.wheel(0, 920)
  81  | 
  82  |     await expect(page).toHaveURL(/\/pitch#mesh-moat$/)
  83  |     await expect(page.getByTestId('pitch-current-marker')).toHaveText('Current frame · 03 / 06')
  84  |   })
  85  | 
  86  |   test('navigation deep links honor known hashes and repair stale hashes', async ({ page }) => {
  87  |     await page.goto('/pitch#mesh-moat')
  88  | 
  89  |     await expect(page).toHaveURL(/\/pitch#mesh-moat$/)
  90  |     await expect(page.getByTestId('pitch-current-marker')).toHaveText('Current frame · 03 / 06')
  91  |     await expect(page.getByTestId('pitch-route-shell')).toHaveAttribute('data-active-slide-id', 'mesh-moat')
  92  |     await expect(page.getByTestId('pitch-agenda-item').nth(2)).toHaveAttribute('aria-current', 'step')
  93  | 
  94  |     await page.goto('/pitch#not-a-real-slide')
  95  | 
  96  |     await expect(page).toHaveURL(/\/pitch#wedge$/)
  97  |     await expect(page.getByTestId('pitch-current-marker')).toHaveText('Current frame · 01 / 06')
  98  |     await expect(page.getByTestId('pitch-route-shell')).toHaveAttribute('data-active-slide-id', 'wedge')
  99  |   })
  100 | 
  101 |   test('navigation indicators jump exactly to the requested slide', async ({ page }) => {
  102 |     await page.goto('/pitch')
  103 | 
  104 |     await page.getByRole('button', { name: /Go to slide 05:/i }).click()
  105 | 
  106 |     await expect(page).toHaveURL(/\/pitch#distribution$/)
  107 |     await expect(page.getByTestId('pitch-current-marker')).toHaveText('Current frame · 05 / 06')
  108 |     await expect(page.getByTestId('pitch-route-shell')).toHaveAttribute(
  109 |       'data-active-slide-id',
  110 |       'distribution',
  111 |     )
  112 |     await expect(page.getByTestId('pitch-slide').nth(4)).toHaveAttribute('data-active', 'true')
  113 |     await expect(page.getByTestId('pitch-agenda-item').nth(4)).toHaveAttribute('aria-current', 'step')
  114 |   })
  115 | })
  116 | 
  117 | test.describe('/pitch export', () => {
  118 |   test.describe('pre-hydration', () => {
  119 |     test.use({ javaScriptEnabled: false })
  120 | 
  121 |     test('export control stays inert before hydration', async ({ page }) => {
  122 |       await page.goto('/pitch')
  123 | 
  124 |       await expect(page.getByTestId('pitch-export-button')).toBeDisabled()
  125 |       await expect(page.getByTestId('pitch-export-button')).toHaveText('Preparing export')
  126 |       await expect(page.getByTestId('pitch-export-status')).toContainText(
  127 |         'Print export arms after hydration so server render stays inert.',
  128 |       )
  129 |     })
  130 |   })
  131 | 
  132 |   test('export button uses browser print once per request from first, middle, and deep-linked slides', async ({ page }) => {
  133 |     await page.addInitScript(() => {
  134 |       let printCalls = 0
  135 | 
  136 |       window.print = () => {
  137 |         printCalls += 1
  138 |       }
  139 | 
  140 |       Object.defineProperty(window, '__pitchPrintCalls', {
  141 |         configurable: true,
  142 |         value: () => printCalls,
  143 |       })
  144 |     })
  145 | 
  146 |     const getPrintCalls = async () =>
  147 |       page.evaluate(() => (window as Window & { __pitchPrintCalls: () => number }).__pitchPrintCalls())
  148 | 
> 149 |     await page.goto('/pitch')
      |                ^ Error: page.goto: Protocol error (Page.navigate): Cannot navigate to invalid URL
  150 | 
  151 |     const exportButton = page.getByTestId('pitch-export-button')
  152 | 
  153 |     await expect(exportButton).toHaveAttribute('data-export-state', 'ready')
  154 |     await exportButton.click()
  155 |     await exportButton.click()
  156 | 
  157 |     await expect.poll(getPrintCalls).toBe(1)
  158 |     await expect(exportButton).toHaveAttribute('data-export-state', 'printing')
  159 | 
  160 |     await page.evaluate(() => {
  161 |       window.dispatchEvent(new Event('afterprint'))
  162 |     })
  163 | 
  164 |     await expect(exportButton).toHaveAttribute('data-export-state', 'ready')
  165 |     await expect(page).toHaveURL(/\/pitch#wedge$/)
  166 | 
  167 |     await page.getByRole('button', { name: /Go to slide 05:/i }).click()
  168 |     await expect(page).toHaveURL(/\/pitch#distribution$/)
  169 | 
  170 |     await exportButton.click()
  171 |     await expect.poll(getPrintCalls).toBe(2)
  172 |     await page.evaluate(() => {
  173 |       window.dispatchEvent(new Event('afterprint'))
  174 |     })
  175 |     await expect(exportButton).toHaveAttribute('data-export-state', 'ready')
  176 | 
  177 |     await page.goto('/pitch#mesh-moat')
  178 |     await expect(page).toHaveURL(/\/pitch#mesh-moat$/)
  179 | 
  180 |     await exportButton.click()
  181 |     await expect.poll(getPrintCalls).toBe(3)
  182 |   })
  183 | 
  184 |   test('print media hides interactive chrome and keeps every slide readable in order', async ({ page }) => {
  185 |     await page.goto('/pitch#distribution')
  186 |     await page.emulateMedia({ media: 'print' })
  187 | 
  188 |     await expect(page.locator('header')).toBeHidden()
  189 |     await expect(page.locator('footer')).toBeHidden()
  190 |     await expect(page.getByTestId('pitch-controls')).toBeHidden()
  191 |     await expect(page.getByTestId('pitch-current-marker')).toBeHidden()
  192 | 
  193 |     await expect(page.getByTestId('pitch-slide')).toHaveCount(pitchDeck.slides.length)
  194 | 
  195 |     for (const slide of pitchDeck.slides) {
  196 |       await expect(page.getByRole('heading', { name: slide.title })).toBeVisible()
  197 |       await expect(page.getByText(slide.summary, { exact: true })).toBeVisible()
  198 |     }
  199 |   })
  200 | })
  201 | 
```