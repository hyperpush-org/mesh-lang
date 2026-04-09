<script setup lang="ts">
import { Sheet, SheetContent, SheetTitle } from '@/components/ui/sheet'
import { useSidebar, type SidebarItem } from '@/composables/useSidebar'
import DocsSidebar from './DocsSidebar.vue'

defineProps<{
  items: SidebarItem[]
}>()

const { isOpen } = useSidebar()
</script>

<template>
  <Sheet v-model:open="isOpen">
    <SheetContent side="left" class="w-72 p-0">
      <SheetTitle class="sr-only">Navigation</SheetTitle>
      <DocsSidebar :items="items" />
    </SheetContent>
  </Sheet>
</template>
