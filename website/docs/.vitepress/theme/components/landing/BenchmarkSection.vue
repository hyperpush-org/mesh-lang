<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useScrollReveal } from '@/composables/useScrollReveal'
import { Activity, Server, Clock, Cpu } from 'lucide-vue-next'

const { observe } = useScrollReveal()
const section = ref<HTMLElement>()
const barsVisible = ref(false)
const activeTab = ref<'throughput' | 'latency' | 'memory'>('throughput')

// Real benchmark data from RESULTS.md — Isolated Peak Throughput
const throughputData = [
  { lang: 'Rust', value: 46244, color: '#dea584', textColor: '#dea584', icon: '🦀' },
  { lang: 'Go', value: 30306, color: '#00ADD8', textColor: '#00ADD8', icon: '🔵' },
  { lang: 'Mesh', value: 29108, color: '#f5f5f5', textColor: 'var(--foreground)', icon: '◆', highlight: true },
  { lang: 'Elixir', value: 12441, color: '#9B59B6', textColor: '#9B59B6', icon: '💧' },
]

const latencyData = [
  { lang: 'Rust', p50: 2.06, p99: 4.55, color: '#dea584', textColor: '#dea584' },
  { lang: 'Mesh', p50: 2.77, p99: 16.94, color: '#f5f5f5', textColor: 'var(--foreground)', highlight: true },
  { lang: 'Go', p50: 2.95, p99: 8.51, color: '#00ADD8', textColor: '#00ADD8' },
  { lang: 'Elixir', p50: 6.74, p99: 25.14, color: '#9B59B6', textColor: '#9B59B6' },
]

const memoryData = [
  { lang: 'Go', value: 1.5, color: '#00ADD8', textColor: '#00ADD8' },
  { lang: 'Elixir', value: 1.6, color: '#9B59B6', textColor: '#9B59B6' },
  { lang: 'Rust', value: 3.4, color: '#dea584', textColor: '#dea584' },
  { lang: 'Mesh', value: 4.9, color: '#f5f5f5', textColor: 'var(--foreground)', highlight: true },
]

const maxThroughput = computed(() => Math.max(...throughputData.map(d => d.value)))
const maxLatency = computed(() => Math.max(...latencyData.map(d => d.p99)))
const maxMemory = computed(() => Math.max(...memoryData.map(d => d.value)))

function formatNumber(n: number): string {
  return n.toLocaleString('en-US')
}

onMounted(() => {
  if (section.value) {
    const io = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          barsVisible.value = true
          io.disconnect()
        }
      },
      { threshold: 0.2 },
    )
    io.observe(section.value)
    observe(section.value)
  }
})
</script>

<template>
  <section ref="section" class="relative border-t border-border py-20 md:py-28 overflow-hidden">
    <!-- Subtle grid background -->
    <div class="absolute inset-0 opacity-[0.03] dark:opacity-[0.05]" style="background-image: linear-gradient(var(--foreground) 1px, transparent 1px), linear-gradient(90deg, var(--foreground) 1px, transparent 1px); background-size: 60px 60px;" />

    <div class="relative mx-auto max-w-5xl px-4">
      <!-- Section header -->
      <div class="text-center">
        <div class="inline-flex items-center gap-2 rounded-full border border-border bg-card px-3.5 py-1.5 text-xs font-medium text-muted-foreground mb-4">
          <Activity class="size-3.5" />
          Real-world benchmarks
        </div>
        <h2 class="mt-3 text-3xl font-bold tracking-tight text-foreground sm:text-4xl lg:text-5xl">
          Native speed, expressive as Elixir
        </h2>
        <p class="mx-auto mt-4 max-w-xl text-lg text-muted-foreground">
          Benchmarked on dedicated Fly.io machines — 2 vCPU, 4 GB RAM, same region, private network. No synthetic games.
        </p>
        <!-- Elixir callout -->
        <div class="mt-6 inline-flex items-start gap-2.5 rounded-xl border border-border bg-muted/40 px-4 py-3 text-left max-w-lg">
          <span class="mt-px text-sm">💧</span>
          <p class="text-sm text-muted-foreground leading-relaxed">
            <span class="font-semibold text-foreground">The meaningful number is Mesh vs Elixir.</span>
            They share the same actor model — Mesh gets you <span class="font-medium text-foreground">2.3× the throughput</span> at less than half the latency, compiled to a native binary.
          </p>
        </div>
      </div>

      <!-- Tab switcher -->
      <div class="mt-10 flex items-center justify-center sm:mt-12">
        <div class="grid w-full max-w-md grid-cols-3 rounded-lg border border-border bg-card p-1 gap-1">
          <button
            @click="activeTab = 'throughput'"
            class="flex items-center justify-center gap-1 rounded-md px-2 py-2 text-xs font-medium transition-all duration-200 sm:gap-1.5 sm:px-4 sm:text-sm"
            :class="activeTab === 'throughput' ? 'bg-foreground text-background shadow-sm' : 'text-muted-foreground hover:text-foreground'"
          >
            <Server class="size-3.5" />
            Throughput
          </button>
          <button
            @click="activeTab = 'latency'"
            class="flex items-center justify-center gap-1 rounded-md px-2 py-2 text-xs font-medium transition-all duration-200 sm:gap-1.5 sm:px-4 sm:text-sm"
            :class="activeTab === 'latency' ? 'bg-foreground text-background shadow-sm' : 'text-muted-foreground hover:text-foreground'"
          >
            <Clock class="size-3.5" />
            Latency
          </button>
          <button
            @click="activeTab = 'memory'"
            class="flex items-center justify-center gap-1 rounded-md px-2 py-2 text-xs font-medium transition-all duration-200 sm:gap-1.5 sm:px-4 sm:text-sm"
            :class="activeTab === 'memory' ? 'bg-foreground text-background shadow-sm' : 'text-muted-foreground hover:text-foreground'"
          >
            <Cpu class="size-3.5" />
            Memory
          </button>
        </div>
      </div>

      <!-- Throughput chart -->
      <div v-show="activeTab === 'throughput'" class="mt-8 sm:mt-10">
        <div class="rounded-xl border border-border bg-card p-6 md:p-8 shadow-lg">
          <div class="flex items-baseline justify-between mb-8">
            <div>
              <h3 class="text-lg font-semibold text-foreground">Requests per second</h3>
              <p class="text-sm text-muted-foreground mt-0.5">GET /text — Isolated VMs, 100 concurrent connections</p>
            </div>
            <div class="hidden sm:block text-xs text-muted-foreground font-mono bg-muted px-2.5 py-1 rounded-md">
              higher is better
            </div>
          </div>

          <div class="space-y-5">
            <div v-for="(item, index) in throughputData" :key="item.lang" class="group">
              <div class="flex items-center gap-2 sm:gap-4">
                <div class="w-12 shrink-0 text-right sm:w-16">
                  <span
                    class="text-xs font-semibold sm:text-sm"
                    :class="item.highlight ? 'text-foreground' : 'text-muted-foreground'"
                  >{{ item.lang }}</span>
                </div>

                <div class="flex-1 relative h-10 rounded-lg bg-muted/50 overflow-hidden">
                  <div
                    class="absolute inset-y-0 left-0 rounded-lg transition-all duration-200 group-hover:brightness-110"
                    :class="[barsVisible ? 'bar-animated' : '']"
                    :style="{
                      '--bar-scale': item.value / maxThroughput,
                      backgroundColor: item.highlight ? 'var(--foreground)' : item.color,
                      opacity: item.highlight ? 1 : 0.8,
                      animationDelay: `${index * 150}ms`,
                      width: '100%',
                    }"
                  />
                  <!-- Mesh highlight glow -->
                  <div
                    v-if="item.highlight"
                    class="absolute inset-y-0 left-0 rounded-lg bg-foreground/20 blur-md"
                    :class="[barsVisible ? 'bar-animated' : '']"
                    :style="{
                      '--bar-scale': item.value / maxThroughput,
                      animationDelay: `${index * 150}ms`,
                      width: '100%',
                    }"
                  />
                </div>

                <div class="w-16 shrink-0 text-right sm:w-20">
                  <span
                    class="text-xs font-mono font-bold tabular-nums sm:text-sm"
                    :class="[
                      item.highlight ? 'text-foreground' : 'text-muted-foreground',
                      barsVisible ? 'counter-animated' : 'opacity-0',
                    ]"
                    :style="{ animationDelay: `${0.8 + index * 0.15}s` }"
                  >{{ formatNumber(item.value) }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Latency chart -->
      <div v-show="activeTab === 'latency'" class="mt-8 sm:mt-10">
        <div class="rounded-xl border border-border bg-card p-6 md:p-8 shadow-lg">
          <div class="flex items-baseline justify-between mb-8">
            <div>
              <h3 class="text-lg font-semibold text-foreground">Response latency</h3>
              <p class="text-sm text-muted-foreground mt-0.5">GET /text — p50 and p99 percentiles</p>
            </div>
            <div class="hidden sm:block text-xs text-muted-foreground font-mono bg-muted px-2.5 py-1 rounded-md">
              lower is better
            </div>
          </div>

          <div class="space-y-6">
            <div v-for="item in latencyData" :key="item.lang" class="group">
              <div class="mb-1.5 flex items-center gap-2 sm:gap-4">
                <div class="w-12 shrink-0 text-right sm:w-16">
                  <span
                    class="text-xs font-semibold sm:text-sm"
                    :class="item.highlight ? 'text-foreground' : 'text-muted-foreground'"
                  >{{ item.lang }}</span>
                </div>
                <div class="flex-1" />
                <div class="shrink-0 flex items-center gap-2 text-[11px] font-mono tabular-nums sm:gap-4 sm:text-xs">
                  <span class="text-muted-foreground">p50 <strong :style="{ color: item.highlight ? 'var(--foreground)' : item.textColor }">{{ item.p50 }}ms</strong></span>
                  <span class="text-muted-foreground">p99 <strong :style="{ color: item.highlight ? 'var(--foreground)' : item.textColor }">{{ item.p99 }}ms</strong></span>
                </div>
              </div>
              <div class="flex items-center gap-2 sm:gap-4">
                <div class="w-12 shrink-0 sm:w-16" />
                <div class="flex-1 relative h-3 rounded-full bg-muted/50 overflow-hidden">
                  <!-- p99 bar (background) -->
                  <div
                    class="absolute inset-y-0 left-0 rounded-full opacity-30"
                    :style="{
                      width: `${(item.p99 / maxLatency) * 100}%`,
                      backgroundColor: item.highlight ? 'var(--foreground)' : item.color,
                    }"
                  />
                  <!-- p50 bar (foreground) -->
                  <div
                    class="absolute inset-y-0 left-0 rounded-full transition-all duration-200 group-hover:brightness-110"
                    :style="{
                      width: `${(item.p50 / maxLatency) * 100}%`,
                      backgroundColor: item.highlight ? 'var(--foreground)' : item.color,
                      opacity: item.highlight ? 1 : 0.8,
                    }"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Memory chart -->
      <div v-show="activeTab === 'memory'" class="mt-8 sm:mt-10">
        <div class="rounded-xl border border-border bg-card p-6 md:p-8 shadow-lg">
          <div class="flex items-baseline justify-between mb-8">
            <div>
              <h3 class="text-lg font-semibold text-foreground">Peak RSS at startup</h3>
              <p class="text-sm text-muted-foreground mt-0.5">Memory footprint before load</p>
            </div>
            <div class="hidden sm:block text-xs text-muted-foreground font-mono bg-muted px-2.5 py-1 rounded-md">
              lower is better
            </div>
          </div>

          <div class="space-y-5">
            <div v-for="item in memoryData" :key="item.lang" class="group">
              <div class="flex items-center gap-2 sm:gap-4">
                <div class="w-12 shrink-0 text-right sm:w-16">
                  <span
                    class="text-xs font-semibold sm:text-sm"
                    :class="item.highlight ? 'text-foreground' : 'text-muted-foreground'"
                  >{{ item.lang }}</span>
                </div>

                <div class="flex-1 relative h-10 rounded-lg bg-muted/50 overflow-hidden">
                  <div
                    class="absolute inset-y-0 left-0 rounded-lg transition-all duration-200 group-hover:brightness-110"
                    :style="{
                      width: `${(item.value / maxMemory) * 100}%`,
                      backgroundColor: item.highlight ? 'var(--foreground)' : item.color,
                      opacity: item.highlight ? 1 : 0.8,
                    }"
                  />
                </div>

                <div class="w-14 shrink-0 text-right sm:w-16">
                  <span
                    class="text-xs font-mono font-bold tabular-nums sm:text-sm"
                    :class="item.highlight ? 'text-foreground' : 'text-muted-foreground'"
                  >{{ item.value }} MB</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Methodology note -->
      <div class="mt-6 text-center">
        <p class="text-xs text-muted-foreground">
          Fly.io <code class="text-xs bg-muted px-1.5 py-0.5 rounded font-mono">performance-2x</code> · 100 connections · 30s warmup + 5×30s runs · Run 1 excluded ·
          <a href="https://github.com/hyperpush-org/mesh-lang/blob/main/benchmarks/METHODOLOGY.md" class="underline underline-offset-2 hover:text-foreground transition-colors">Full methodology →</a>
        </p>
      </div>
    </div>
  </section>
</template>
