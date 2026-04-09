<script setup lang="ts">
import type { OutlineItem } from '@/composables/useOutline'

defineProps<{
  headers: OutlineItem[]
  root?: boolean
  activeId?: string
}>()
</script>

<template>
  <ul class="border-l border-border">
    <li v-for="item in headers" :key="item.link">
      <a
        :href="item.link"
        class="block py-1 pl-3 text-[13px] -ml-px border-l-2 transition-all duration-200"
        :class="[
          activeId === item.link.slice(1)
            ? 'border-foreground text-foreground font-medium'
            : 'border-transparent text-muted-foreground hover:text-foreground hover:border-foreground/50',
        ]"
      >
        {{ item.title }}
      </a>
      <DocsOutlineItem
        v-if="item.children?.length"
        :headers="item.children"
        :root="false"
        :active-id="activeId"
        class="ml-3"
      />
    </li>
  </ul>
</template>
