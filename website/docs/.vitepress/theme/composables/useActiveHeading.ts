import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vitepress'

export function useActiveHeading() {
  const activeId = ref('')
  const route = useRoute()
  let observer: IntersectionObserver | null = null

  function observe() {
    if (observer) {
      observer.disconnect()
    }

    const headings = document.querySelectorAll('.docs-content h2[id], .docs-content h3[id]')
    if (headings.length === 0) return

    observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            activeId.value = entry.target.id
          }
        }
      },
      {
        rootMargin: '-80px 0px -70% 0px',
      }
    )

    headings.forEach((heading) => observer!.observe(heading))
  }

  onMounted(() => {
    observe()
  })

  watch(() => route.path, () => {
    // Re-observe after route change (DOM updates on next tick)
    setTimeout(observe, 100)
  })

  onUnmounted(() => {
    if (observer) {
      observer.disconnect()
    }
  })

  return { activeId }
}
