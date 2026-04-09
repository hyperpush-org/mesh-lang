# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: mesher/landing/tests/pitch-route.spec.ts >> /pitch route shell >> route shell renders route-local metadata and baseline story markers
- Location: mesher/landing/tests/pitch-route.spec.ts:5:7

# Error details

```
Error: page.goto: Protocol error (Page.navigate): Cannot navigate to invalid URL
Call log:
  - navigating to "/pitch", waiting until "load"

```

# Test source

```ts
  1   | import { expect, test } from '@playwright/test'
  2   | import { pitchDeck } from '../lib/pitch/slides'
  3   | 
  4   | test.describe('/pitch route shell', () => {
  5   |   test('route shell renders route-local metadata and baseline story markers', async ({ page }) => {
> 6   |     await page.goto('/pitch')
      |                ^ Error: page.goto: Protocol error (Page.navigate): Cannot navigate to invalid URL
  7   | 
  8   |     await expect(page).toHaveURL(/\/pitch(?:#wedge)?$/)
  9   |     await expect(page).toHaveTitle(/Pitch Deck — hyperpush on Mesh/)
  10  | 
  11  |     await expect(page.locator('meta[name="description"]')).toHaveAttribute(
  12  |       'content',
  13  |       /Mesh-backed distributed systems behavior/,
  14  |     )
  15  | 
  16  |     await expect(
  17  |       page.getByRole('heading', {
  18  |         name: 'hyperpush sells the workflow. Mesh makes the story durable.',
  19  |       }),
  20  |     ).toBeVisible()
  21  | 
  22  |     await expect(page.getByTestId('pitch-current-marker')).toHaveText('Current frame · 01 / 06')
  23  |     await expect(page.getByTestId('pitch-agenda-item')).toHaveCount(6)
  24  |     await expect(page.getByTestId('pitch-route-shell')).toHaveAttribute('data-active-slide-id', 'wedge')
  25  |     await expect(page.getByTestId('pitch-slide').first()).toHaveAttribute('data-active', 'true')
  26  |     await expect(page.getByTestId('pitch-agenda-item').first()).toHaveAttribute('aria-current', 'step')
  27  |     await expect(page.getByTestId('pitch-slide').first()).toContainText(
  28  |       'Open-source error tracking is the wedge. Mesh-backed reliability is the reason it wins.',
  29  |     )
  30  |     await expect(page.getByTestId('pitch-agenda-item').nth(2)).toContainText(
  31  |       'Mesh is not a footnote in the pitch. It is the moat behind the product.',
  32  |     )
  33  |     await expect(page.getByTestId('pitch-agenda-item').last()).toContainText(
  34  |       'The pitch closes on a product today and a platform tomorrow.',
  35  |     )
  36  |   })
  37  | })
  38  | 
  39  | test.describe('/pitch navigation', () => {
  40  |   test('navigation keyboard input stays bounded and updates inspectable state', async ({ page }) => {
  41  |     await page.goto('/pitch')
  42  | 
  43  |     await page.keyboard.press('KeyA')
  44  |     await expect(page).toHaveURL(/\/pitch#wedge$/)
  45  | 
  46  |     await page.keyboard.press('ArrowLeft')
  47  |     await expect(page).toHaveURL(/\/pitch#wedge$/)
  48  |     await expect(page.getByTestId('pitch-previous')).toBeDisabled()
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
```