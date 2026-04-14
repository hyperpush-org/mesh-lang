<script setup lang="ts">
import { Sheet, SheetContent, SheetTitle } from '@/components/ui/sheet'
import { useSidebar, type SidebarItem } from '@/composables/useSidebar'
import DocsSidebar from './DocsSidebar.vue'

defineProps<{
  items: SidebarItem[]
}>()

const { isOpen } = useSidebar()

const quickLinks = [
  { text: 'Main Page', href: '/', target: '_self' },
  { text: 'Packages', href: 'https://packages.meshlang.dev', target: '_blank' },
  { text: 'GitHub', href: 'https://github.com/hyperpush-org/mesh-lang', target: '_blank' },
]
</script>

<template>
  <Sheet v-model:open="isOpen">
    <SheetContent side="left" class="flex h-full w-72 flex-col overflow-hidden p-0">
      <SheetTitle class="sr-only">Navigation</SheetTitle>
      <div class="border-b border-border px-4 py-4">
        <div class="text-[11px] font-mono uppercase tracking-wider text-muted-foreground">Quick Links</div>
        <nav class="mt-2 flex flex-col gap-1">
          <a
            v-for="link in quickLinks"
            :key="link.href"
            :href="link.href"
            :target="link.target"
            :rel="link.target === '_blank' ? 'noreferrer noopener' : undefined"
            class="rounded-md px-2.5 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
            @click="isOpen = false"
          >
            {{ link.text }}
          </a>
        </nav>
      </div>
      <div class="min-h-0 flex-1">
        <DocsSidebar :items="items" />
      </div>
    </SheetContent>
  </Sheet>
</template>
