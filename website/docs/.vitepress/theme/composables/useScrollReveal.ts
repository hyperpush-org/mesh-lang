import { onMounted, onUnmounted } from 'vue'

export function useScrollReveal() {
  let observer: IntersectionObserver | null = null

  function observe(el: Element) {
    if (observer) {
      observer.observe(el)
    }
  }

  onMounted(() => {
    observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            entry.target.classList.add('is-visible')
            observer?.unobserve(entry.target)
          }
        }
      },
      { threshold: 0.15 },
    )
  })

  onUnmounted(() => {
    observer?.disconnect()
    observer = null
  })

  return { observe }
}
