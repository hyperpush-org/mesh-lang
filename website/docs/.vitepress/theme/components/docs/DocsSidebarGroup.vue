<script setup lang="ts">
import type { SidebarItem } from '@/composables/useSidebar'
import DocsSidebarItem from './DocsSidebarItem.vue'

defineProps<{
  item: SidebarItem
}>()
</script>

<template>
  <div class="mb-5">
    <div class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-2 px-2">
      {{ item.text }}
    </div>
    <ul class="flex flex-col gap-0.5">
      <li v-for="child in item.items" :key="child.text">
        <DocsSidebarItem :item="child" />
      </li>
    </ul>
  </div>
</template>
