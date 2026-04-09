<script setup lang="ts">
import { ref } from 'vue'
import VPNavBarSearch from 'vitepress/dist/client/theme-default/components/VPNavBarSearch.vue'
import { withBase, useData } from 'vitepress'
import ThemeToggle from './ThemeToggle.vue'
import { useSidebar } from '@/composables/useSidebar'
import { Menu, X } from 'lucide-vue-next'

const { hasSidebar, is960, toggle } = useSidebar()
const { isDark } = useData()

// Mobile menu for non-docs pages (landing, packages, etc.)
const mobileMenuOpen = ref(false)

const navLinks = [
  { text: 'Docs', href: '/docs/getting-started/' },
  { text: 'Packages', href: '/packages/' },
  { text: 'GitHub', href: 'https://github.com/hyperpush-org/mesh-lang' },
]
</script>

<template>
  <header class="sticky top-0 z-50 w-full border-b border-border bg-background/80 backdrop-blur-xl">
    <div class="relative mx-auto flex h-14 max-w-[90rem] items-center px-4 lg:px-6">
      <!-- Logo + mobile hamburger -->
      <div class="flex shrink-0 items-center gap-3">
        <!-- Docs sidebar toggle (mobile, inside docs) -->
        <button
          v-if="hasSidebar && !is960"
          class="inline-flex items-center justify-center rounded-md p-2 text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
          aria-label="Toggle sidebar"
          @click="toggle"
        >
          <Menu class="size-5" />
        </button>
        <!-- Mobile menu toggle (outside docs) -->
        <button
          v-if="!hasSidebar || is960"
          class="md:hidden inline-flex items-center justify-center rounded-md p-2 text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
          :aria-label="mobileMenuOpen ? 'Close menu' : 'Open menu'"
          :aria-expanded="mobileMenuOpen"
          @click="mobileMenuOpen = !mobileMenuOpen"
        >
          <X v-if="mobileMenuOpen" class="size-5" />
          <Menu v-else class="size-5" />
        </button>
        <a href="/" class="flex items-center">
          <img :src="withBase(isDark ? '/logo-white.svg' : '/logo-black.svg')" alt="Mesh" class="h-7 w-auto" />
        </a>
      </div>

      <!-- Navigation Links (viewport-centered, desktop) -->
      <nav class="hidden items-center justify-center gap-1 text-sm md:flex absolute inset-0 pointer-events-none">
        <a
          href="/docs/getting-started/"
          class="pointer-events-auto rounded-md px-3 py-1.5 text-muted-foreground transition-colors hover:text-foreground hover:bg-accent"
        >
          Docs
        </a>
        <a
          href="/packages/"
          class="pointer-events-auto rounded-md px-3 py-1.5 text-muted-foreground transition-colors hover:text-foreground hover:bg-accent"
        >
          Packages
        </a>
        <a
          href="https://github.com/hyperpush-org/mesh-lang"
          class="pointer-events-auto rounded-md px-3 py-1.5 text-muted-foreground transition-colors hover:text-foreground hover:bg-accent"
        >
          GitHub
        </a>
      </nav>

      <!-- Search + Theme toggle (right) -->
      <div class="flex shrink-0 items-center gap-1 ml-auto">
        <VPNavBarSearch />
        <ThemeToggle />
      </div>
    </div>

    <!-- Mobile dropdown menu -->
    <div
      v-if="mobileMenuOpen"
      class="md:hidden border-t border-border bg-background/95 backdrop-blur-xl"
    >
      <nav class="mx-auto max-w-[90rem] flex flex-col px-4 py-3 gap-0.5">
        <a
          v-for="link in navLinks"
          :key="link.href"
          :href="link.href"
          class="flex items-center rounded-md px-3 py-2.5 text-sm text-muted-foreground transition-colors hover:text-foreground hover:bg-accent"
          @click="mobileMenuOpen = false"
        >
          {{ link.text }}
        </a>
      </nav>
    </div>
  </header>
</template>
