<script setup lang="ts">
import { useData } from 'vitepress'

const { isDark } = useData()
const currentYear = new Date().getUTCFullYear()

const resources = [
  { label: 'Getting Started', href: '/docs/getting-started/' },
  { label: 'Language Guide', href: '/docs/language-basics/' },
  { label: 'Cheatsheet', href: '/docs/cheatsheet/' },
  { label: 'Standard Library', href: '/docs/stdlib/' },
]

const features = [
  { label: 'Concurrency', href: '/docs/concurrency/' },
  { label: 'Web & HTTP', href: '/docs/web/' },
  { label: 'Databases', href: '/docs/databases/' },
  { label: 'Testing', href: '/docs/testing/' },
]

const community = [
  { label: 'GitHub', href: 'https://github.com/hyperpush-org/mesh-lang', target:'_blank' },
  { label: 'Discussions', href: 'https://github.com/hyperpush-org/mesh-lang/discussions', target: '_blank' },
  { label: 'Packages', href: 'https://packages.meshlang.dev', target: '_blank' },
]
</script>

<template>
  <footer class="border-t border-border bg-muted/20 py-14 md:py-18">
    <div class="mx-auto max-w-5xl px-4">
      <div class="grid gap-10 sm:grid-cols-2 md:grid-cols-4">
        <!-- Brand -->
        <div>
          <div class="flex items-center gap-2.5">
            <img
              :src="isDark ? '/logo-icon-white.svg' : '/logo-icon-black.svg'"
              alt="Mesh"
              class="size-7"
            />
            <span class="text-lg font-bold text-foreground tracking-tight">Mesh</span>
          </div>
          <p class="mt-3 text-sm text-muted-foreground leading-relaxed max-w-[200px]">
            Expressive, concurrent, type-safe programming language compiled to native binaries.
          </p>
        </div>

        <!-- Resources -->
        <div>
          <h3 class="text-xs font-semibold text-foreground uppercase tracking-wider">Resources</h3>
          <ul class="mt-4 space-y-2.5">
            <li v-for="link in resources" :key="link.href">
              <a :href="link.href" class="text-sm text-muted-foreground transition-colors hover:text-foreground">
                {{ link.label }}
              </a>
            </li>
          </ul>
        </div>

        <!-- Features -->
        <div>
          <h3 class="text-xs font-semibold text-foreground uppercase tracking-wider">Features</h3>
          <ul class="mt-4 space-y-2.5">
            <li v-for="link in features" :key="link.href">
              <a :href="link.href" class="text-sm text-muted-foreground transition-colors hover:text-foreground">
                {{ link.label }}
              </a>
            </li>
          </ul>
        </div>

        <!-- Community -->
        <div>
          <h3 class="text-xs font-semibold text-foreground uppercase tracking-wider">Community</h3>
          <ul class="mt-4 space-y-2.5">
            <li v-for="link in community" :key="link.href" >
              <a :href="link.href" :target="link.target" class="text-sm text-muted-foreground transition-colors hover:text-foreground">
                {{ link.label }}
              </a>
            </li>
          </ul>
        </div>
      </div>

      <!-- Bottom bar -->
      <div class="mt-14 flex flex-col sm:flex-row items-center justify-between gap-4 border-t border-border pt-6">
        <p class="text-xs text-muted-foreground">
          &copy; {{ currentYear }} The Mesh Programming Language
        </p>
        <div class="flex items-center gap-4">
          <a href="https://github.com/hyperpush-org/mesh-lang" class="text-xs text-muted-foreground hover:text-foreground transition-colors">
            GitHub
          </a>
          <span class="text-muted-foreground/30">·</span>
          <a href="https://github.com/hyperpush-org/mesh-lang/blob/main/LICENSE" class="text-xs text-muted-foreground hover:text-foreground transition-colors">
            License
          </a>
        </div>
      </div>
    </div>
  </footer>
</template>
