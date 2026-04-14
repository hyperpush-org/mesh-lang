import { computed, ref, watch } from 'vue'
import { useData, useRoute } from 'vitepress'
import { useMediaQuery } from '@vueuse/core'

// Shared mobile sidebar state across NavBar, DocsLayout and MobileSidebar.
const sharedSidebarOpen = ref(false)

export interface SidebarItem {
  text?: string
  link?: string
  items?: SidebarItem[]
  collapsed?: boolean
  base?: string
  docFooterText?: string
  icon?: string
}

export function useSidebar() {
  const { theme, page, frontmatter } = useData()
  const route = useRoute()
  const is960 = useMediaQuery('(min-width: 960px)')

  const sidebar = computed<SidebarItem[]>(() => {
    const sidebarConfig = theme.value.sidebar
    if (!sidebarConfig) return []
    if (Array.isArray(sidebarConfig)) return sidebarConfig

    // Multi-sidebar: find matching path by longest prefix
    const relativePath = page.value.relativePath
    const path = ensureStartingSlash(relativePath)
    const dir = Object.keys(sidebarConfig)
      .sort((a, b) => b.split('/').length - a.split('/').length)
      .find((dir) => path.startsWith(ensureStartingSlash(dir)))
    return dir ? sidebarConfig[dir] : []
  })

  const hasSidebar = computed(() => {
    return (
      frontmatter.value.sidebar !== false &&
      sidebar.value.length > 0 &&
      frontmatter.value.layout !== 'home'
    )
  })

  // Auto-close mobile sidebar on route change
  watch(
    () => route.path,
    () => {
      sharedSidebarOpen.value = false
    },
  )

  function open() {
    sharedSidebarOpen.value = true
  }
  function close() {
    sharedSidebarOpen.value = false
  }
  function toggle() {
    sharedSidebarOpen.value ? close() : open()
  }

  return {
    sidebar,
    hasSidebar,
    isOpen: sharedSidebarOpen,
    is960,
    open,
    close,
    toggle,
  }
}

function ensureStartingSlash(path: string): string {
  return path.startsWith('/') ? path : `/${path}`
}

/**
 * Check if a link matches the current page.
 * Normalizes both paths by stripping index.md, .md, .html extensions
 * and trailing slashes before comparison.
 */
export function isActive(currentPath: string, matchPath?: string): boolean {
  if (!matchPath) return false
  const normalizedCurrent = ensureStartingSlash(
    currentPath.replace(/(index)?\.(md|html)$/, ''),
  )
  const normalizedMatch = ensureStartingSlash(
    matchPath.replace(/(index)?\.(md|html)$/, '').replace(/\/$/, ''),
  )
  return (
    normalizedCurrent === normalizedMatch ||
    normalizedCurrent.startsWith(normalizedMatch + '/')
  )
}
