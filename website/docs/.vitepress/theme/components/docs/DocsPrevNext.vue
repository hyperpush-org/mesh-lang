<script setup lang="ts">
import { usePrevNext } from '@/composables/usePrevNext'
import { ChevronLeft, ChevronRight } from 'lucide-vue-next'

const prevNext = usePrevNext()
</script>

<template>
  <div class="not-prose grid grid-cols-2 gap-4">
    <!-- Previous link -->
    <a
      v-if="prevNext.prev?.link"
      :href="prevNext.prev.link"
      class="group flex items-center gap-3 rounded-xl border border-border p-5 text-sm transition-all duration-200 hover:-translate-y-0.5 hover:shadow-lg hover:border-foreground/20"
    >
      <ChevronLeft class="size-4 shrink-0 text-muted-foreground transition-transform duration-200 group-hover:-translate-x-0.5" />
      <div class="min-w-0">
        <div class="text-xs text-muted-foreground mb-1">Previous</div>
        <div class="font-medium text-foreground truncate">{{ prevNext.prev.text }}</div>
      </div>
    </a>
    <span v-else />

    <!-- Next link -->
    <a
      v-if="prevNext.next?.link"
      :href="prevNext.next.link"
      class="group flex items-center justify-end gap-3 rounded-xl border border-border p-5 text-sm transition-all duration-200 hover:-translate-y-0.5 hover:shadow-lg hover:border-foreground/20"
      :class="{ 'col-start-2': !prevNext.prev?.link }"
    >
      <div class="min-w-0 text-right">
        <div class="text-xs text-muted-foreground mb-1">Next</div>
        <div class="font-medium text-foreground truncate">{{ prevNext.next.text }}</div>
      </div>
      <ChevronRight class="size-4 shrink-0 text-muted-foreground transition-transform duration-200 group-hover:translate-x-0.5" />
    </a>
  </div>
</template>
