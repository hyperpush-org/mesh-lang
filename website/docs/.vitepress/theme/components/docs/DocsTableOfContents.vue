<script setup lang="ts">
import { useData } from 'vitepress'
import { useOutline } from '@/composables/useOutline'
import { useActiveHeading } from '@/composables/useActiveHeading'
import { ScrollArea } from '@/components/ui/scroll-area'
import DocsOutlineItem from './DocsOutlineItem.vue'

const { theme } = useData()
const { headers } = useOutline()
const { activeId } = useActiveHeading()

const label = theme.value.outline?.label ?? 'On this page'
</script>

<template>
  <nav v-if="headers.length > 0" class="h-full pr-4">
    <ScrollArea class="h-full">
      <div class="pt-5 pb-5">
        <div class="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">{{ label }}</div>
        <DocsOutlineItem :headers="headers" :active-id="activeId" />
      </div>
    </ScrollArea>
  </nav>
</template>
