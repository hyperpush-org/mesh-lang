import { shallowRef } from 'vue'
import { onContentUpdated } from 'vitepress'
import { useData } from 'vitepress'

export interface OutlineItem {
  element: HTMLElement
  title: string
  link: string
  level: number
  children: OutlineItem[]
}

export function useOutline() {
  const { theme, frontmatter } = useData()
  const headers = shallowRef<OutlineItem[]>([])

  onContentUpdated(() => {
    const outlineConfig = frontmatter.value.outline ?? theme.value.outline
    headers.value = getHeaders(outlineConfig)
  })

  return { headers }
}

export function getHeaders(range?: unknown): OutlineItem[] {
  const allHeaders = [
    ...document.querySelectorAll(
      '.docs-content :where(h1,h2,h3,h4,h5,h6)',
    ),
  ]
    .filter((el) => el.id && el.hasChildNodes())
    .map((el) => {
      const level = Number(el.tagName[1])
      return {
        element: el as HTMLElement,
        title: serializeHeader(el as HTMLElement),
        link: '#' + el.id,
        level,
        children: [] as OutlineItem[],
      }
    })

  // Parse range config: number, [high, low], 'deep', or { level: ... }
  const levelsRange =
    typeof range === 'object' && !Array.isArray(range) && range !== null
      ? (range as Record<string, unknown>).level
      : range
  const resolved = levelsRange || 2
  const [high, low]: [number, number] =
    typeof resolved === 'number'
      ? [resolved, resolved]
      : resolved === 'deep'
        ? [2, 6]
        : (resolved as [number, number])

  return buildTree(allHeaders, high, low)
}

/**
 * Serialize a heading element's text content, skipping anchor links
 * and elements marked with the ignore-header class.
 */
function serializeHeader(h: HTMLElement): string {
  let ret = ''
  for (const node of h.childNodes) {
    if (node.nodeType === 1) {
      if (
        /header-anchor|ignore-header/.test((node as Element).className)
      ) {
        continue
      }
      ret += node.textContent
    } else if (node.nodeType === 3) {
      ret += node.textContent
    }
  }
  return ret.trim()
}

/**
 * Build a nested tree of outline items from a flat list using a
 * stack-based algorithm. Items outside the [min, max] level range
 * are excluded.
 */
function buildTree(
  data: OutlineItem[],
  min: number,
  max: number,
): OutlineItem[] {
  const result: OutlineItem[] = []
  const stack: OutlineItem[] = []
  for (const item of data) {
    const node = { ...item, children: [] }
    if (node.level > max || node.level < min) continue
    let parent = stack[stack.length - 1]
    while (parent && parent.level >= node.level) {
      stack.pop()
      parent = stack[stack.length - 1]
    }
    if (parent) parent.children.push(node)
    else result.push(node)
    stack.push(node)
  }
  return result
}
