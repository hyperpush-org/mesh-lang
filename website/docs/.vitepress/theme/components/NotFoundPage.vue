<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { ArrowLeft } from 'lucide-vue-next'
</script>

<template>
  <main class="flex min-h-[calc(100vh-3.5rem)] flex-col items-center justify-center px-4 text-center">
    <div class="text-8xl font-bold tracking-tighter text-foreground" style="letter-spacing: -0.05em;">404</div>
    <p class="mt-4 text-lg text-muted-foreground">Page not found</p>
    <p class="mt-2 text-sm text-muted-foreground">The page you're looking for doesn't exist or has been moved.</p>
    <Button as="a" href="/" variant="outline" class="mt-8">
      <ArrowLeft class="size-4" />
      Back to home
    </Button>
  </main>
</template>
