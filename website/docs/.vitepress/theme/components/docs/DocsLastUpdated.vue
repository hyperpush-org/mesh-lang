<script setup lang="ts">
import { useData } from 'vitepress'
import { computed } from 'vue'

const { page } = useData()

const lastUpdated = computed(() => {
  const timestamp = page.value.lastUpdated
  if (!timestamp) return null
  return new Date(timestamp).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    timeZone: 'UTC',
  })
})
</script>

<template>
  <span v-if="lastUpdated" class="text-sm text-muted-foreground">
    Last updated: {{ lastUpdated }}
  </span>
</template>
