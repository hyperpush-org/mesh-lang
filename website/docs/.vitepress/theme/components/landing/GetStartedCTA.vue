<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Button } from '@/components/ui/button'
import { ArrowRight, Copy, Check, Terminal } from 'lucide-vue-next'
import { useScrollReveal } from '@/composables/useScrollReveal'

const { observe } = useScrollReveal()
const section = ref<HTMLElement>()
const copied = ref(false)

const installCommand = 'curl -sSf https://meshlang.dev/install.sh | sh'

async function copyCommand() {
  try {
    await navigator.clipboard.writeText(installCommand)
    copied.value = true
    setTimeout(() => { copied.value = false }, 2000)
  } catch {
    // Clipboard API not available
  }
}

onMounted(() => {
  if (section.value) observe(section.value)
})
</script>

<template>
  <section class="relative border-t border-border py-24 md:py-32 overflow-hidden">
    <!-- Background -->
    <div class="absolute inset-0 bg-[radial-gradient(ellipse_60%_40%_at_50%_50%,var(--muted),transparent_70%)] opacity-50" />
    <div class="absolute inset-0 opacity-[0.02] dark:opacity-[0.04]" style="background-image: linear-gradient(var(--foreground) 1px, transparent 1px), linear-gradient(90deg, var(--foreground) 1px, transparent 1px); background-size: 48px 48px;" />

    <div ref="section" class="reveal relative mx-auto max-w-2xl px-4 text-center">
      <div class="mb-6 inline-flex items-center gap-2 rounded-full border border-border bg-card/80 backdrop-blur-sm px-3.5 py-1.5 text-xs font-medium text-muted-foreground shadow-sm">
        <Terminal class="size-3.5" />
        One command to install
      </div>

      <h2 class="text-3xl font-bold tracking-tight text-foreground sm:text-4xl lg:text-5xl">
        Start building your distributed system
      </h2>
      <p class="mt-4 text-base text-muted-foreground sm:text-lg">
        Install Mesh in seconds. Works on macOS, Linux, and Windows.
      </p>

      <!-- Install command -->
      <div class="mx-auto mt-8 flex max-w-xl flex-col items-start gap-3 rounded-xl border border-border bg-card px-4 py-4 font-mono text-sm shadow-lg ring-1 ring-foreground/[0.03] sm:flex-row sm:items-center sm:px-5">
        <span class="text-emerald-500 select-none font-bold">$</span>
        <code class="flex-1 text-left text-foreground break-all sm:truncate">{{ installCommand }}</code>
        <button
          @click="copyCommand"
          class="shrink-0 self-end rounded-md p-1.5 text-muted-foreground transition-all hover:bg-muted hover:text-foreground sm:self-auto"
          :title="copied ? 'Copied!' : 'Copy to clipboard'"
        >
          <Check v-if="copied" class="size-4 text-emerald-500" />
          <Copy v-else class="size-4" />
        </button>
      </div>

      <!-- CTA buttons -->
      <div class="mt-8 flex flex-col items-stretch justify-center gap-3 sm:flex-row sm:items-center">
        <Button as="a" href="/docs/getting-started/" size="lg" class="h-12 w-full px-8 rounded-lg text-base font-semibold shadow-md hover:shadow-lg transition-shadow sm:w-auto">
          Get Started
          <ArrowRight class="ml-1.5 size-4" />
        </Button>
        <Button as="a" href="/docs/distributed/" variant="outline" size="lg" class="h-12 w-full px-8 rounded-lg text-base font-semibold sm:w-auto">
          Distributed Docs
        </Button>
      </div>
    </div>
  </section>
</template>
