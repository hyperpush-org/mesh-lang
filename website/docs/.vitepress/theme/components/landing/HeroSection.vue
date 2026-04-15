<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useData } from 'vitepress'
import { Button } from '@/components/ui/button'
import { getHighlighter, highlightCode } from '@/composables/useShiki'
import { ArrowRight, Github } from 'lucide-vue-next'

const { theme } = useData()

const highlightedHtml = ref('')

const heroCode = `# work.mpl
@cluster pub fn add() -> Int do
  1 + 1
end

# api/router.mpl
from Api.Todos import handle_get_todo, handle_list_todos

pub fn build_router() do
  HTTP.router()
    |> HTTP.on_get("/todos", HTTP.clustered(handle_list_todos))
    |> HTTP.on_get("/todos/:id", HTTP.clustered(handle_get_todo))
end`

onMounted(async () => {
  try {
    const hl = await getHighlighter()
    highlightedHtml.value = highlightCode(hl, heroCode)
  } catch {
    // Highlighting failed -- raw code fallback remains visible
  }
})
</script>

<template>
  <section class="relative overflow-x-clip">
    <!-- Layered background: radial vignette + subtle grid -->
    <div
      class="absolute inset-0 bg-[radial-gradient(ellipse_80%_50%_at_50%_-20%,var(--border),transparent_70%)] opacity-60" />
    <div class="absolute inset-0 opacity-[0.02] dark:opacity-[0.04]"
      style="background-image: linear-gradient(var(--foreground) 1px, transparent 1px), linear-gradient(90deg, var(--foreground) 1px, transparent 1px); background-size: 48px 48px;" />

    <div class="relative mx-auto w-full max-w-6xl px-3 pt-10 pb-14 sm:px-6 sm:pt-16 sm:pb-20 md:pt-20 md:pb-28 lg:pt-28">
      <div class="grid items-center gap-8 sm:gap-12 lg:grid-cols-[1fr_1.1fr] lg:gap-16">
        <!-- Left column: text -->
        <div class="min-w-0 text-center animate-fade-in-up lg:text-left">
          <!-- Version badge -->
          <div
            class="mb-6 inline-flex max-w-full flex-wrap items-center justify-center gap-2 rounded-full border border-border bg-card/80 px-3 py-1.5 text-center text-[11px] font-medium text-muted-foreground shadow-sm backdrop-blur-sm sm:mb-8 sm:px-3.5 sm:text-xs lg:justify-start">
            <span class="relative inline-flex size-2">
              <span class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-500/50" />
              <span class="relative inline-block size-2 rounded-full bg-emerald-500" />
            </span>
            Now in development &mdash; v{{ theme.meshVersion }}
          </div>

          <h1 class="text-[clamp(1.7rem,8vw,2.45rem)] font-extrabold tracking-tight text-foreground sm:text-5xl lg:text-[4.25rem]"
            style="line-height: 1.1; text-wrap: balance;">
            The language built for
            <span class="relative mt-1 block sm:mt-0 sm:inline-block">
              distributed systems.
              <svg class="absolute -bottom-1 left-0 hidden h-3 w-full text-foreground/15 sm:block" viewBox="0 0 200 12"
                preserveAspectRatio="none">
                <path d="M0 9 Q50 0 100 7 Q150 14 200 5" stroke="currentColor" stroke-width="3" fill="none"
                  stroke-linecap="round" />
              </svg>
            </span>
          </h1>

          <p class="mx-auto mt-5 max-w-lg text-base text-muted-foreground sm:mt-6 sm:text-xl lg:mx-0" style="line-height: 1.7;">
            One annotation to distribute work across a fleet. Built-in failover, load balancing, and everything a server
            needs — no orchestration layer required.
          </p>

          <div class="mt-8 flex flex-col items-stretch justify-center gap-3 sm:mt-10 sm:flex-row sm:items-center lg:justify-start">
            <Button as="a" href="/docs/getting-started/" size="lg"
              class="h-12 w-full px-8 rounded-lg text-base font-semibold shadow-md hover:shadow-lg transition-shadow sm:w-auto">
              Get Started
              <ArrowRight class="ml-1.5 size-4" />
            </Button>
            <Button as="a" href="https://github.com/hyperpush-org/mesh-lang" variant="outline" size="lg"
              class="h-12 w-full px-8 rounded-lg text-base font-semibold sm:w-auto">
              <Github class="mr-1.5 size-4" />
              GitHub
            </Button>
          </div>
        </div>

        <!-- Right column: code block -->
        <div class="relative min-w-0 w-full animate-fade-in-up" style="animation-delay: 200ms;">
          <!-- Terminal -->
          <div
            class="relative w-full max-w-full overflow-hidden rounded-xl border border-border bg-card shadow-2xl ring-1 ring-foreground/[0.05]">
            <!-- Terminal header -->
            <div class="flex items-center gap-2 border-b border-border px-4 py-3 bg-muted/30">
              <div class="flex gap-1.5">
                <div class="size-3 rounded-full bg-[#ff5f57] shadow-[inset_0_-1px_0_rgba(0,0,0,0.12)]" />
                <div class="size-3 rounded-full bg-[#febc2e] shadow-[inset_0_-1px_0_rgba(0,0,0,0.12)]" />
                <div class="size-3 rounded-full bg-[#28c840] shadow-[inset_0_-1px_0_rgba(0,0,0,0.12)]" />
              </div>
              <span class="ml-2 text-xs text-muted-foreground font-medium">clustered starter</span>
            </div>
            <!-- Code content -->
            <div v-if="highlightedHtml" v-html="highlightedHtml"
              class="vp-code w-full max-w-full [&_pre]:max-w-full [&_pre]:overflow-x-auto [&_pre]:px-4 [&_pre]:py-4 sm:[&_pre]:px-5 [&_pre]:!bg-transparent" />
            <pre v-else
              class="max-w-full overflow-x-auto px-4 py-4 text-xs leading-relaxed text-foreground font-mono sm:px-5 sm:text-sm"><code>{{ heroCode }}</code></pre>
          </div>

          <!-- Floating language tag -->
          <div
            class="absolute -bottom-2 right-2 hidden rounded-lg border border-border bg-card px-2.5 py-1 text-[11px] shadow-lg font-mono text-muted-foreground animate-float sm:block sm:-bottom-3 sm:-right-3 sm:px-3 sm:py-1.5 sm:text-xs md:-bottom-4 md:-right-4"
            style="animation-delay: 1s;">
            .mpl
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
