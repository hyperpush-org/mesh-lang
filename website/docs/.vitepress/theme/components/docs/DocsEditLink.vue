<script setup lang="ts">
import { useData } from 'vitepress'
import { computed } from 'vue'
import { Pencil } from 'lucide-vue-next'

const { theme, page } = useData()

const editLink = computed(() => {
  const { editLink } = theme.value
  if (!editLink?.pattern) return null
  const url = editLink.pattern.replace(':path', page.value.relativePath)
  return { url, text: editLink.text || 'Edit this page' }
})
</script>

<template>
  <a
    v-if="editLink"
    :href="editLink.url"
    target="_blank"
    rel="noopener noreferrer"
    class="inline-flex items-center gap-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
  >
    <Pencil class="size-3.5" />
    {{ editLink.text }}
  </a>
</template>
