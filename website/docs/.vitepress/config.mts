import { defineConfig, type HeadConfig, type PageData } from 'vitepress'
import tailwindcss from '@tailwindcss/vite'
import path from 'node:path'
import meshGrammar from '../../../tools/editors/vscode-mesh/syntaxes/mesh.tmLanguage.json'
import meshLight from './theme/shiki/mesh-light.json'
import meshDark from './theme/shiki/mesh-dark.json'

const SITE_URL = 'https://meshlang.dev'
const SITE_NAME = 'Mesh Programming Language'
const DEFAULT_DESCRIPTION = 'One public app URL fronts multiple Mesh nodes. Runtime placement stays server-side, and operator truth stays on meshc cluster.'
const SOCIAL_IMAGE_PATH = '/og-image-v2.png'
const SOCIAL_IMAGE_URL = `${SITE_URL}${SOCIAL_IMAGE_PATH}`
const SOCIAL_IMAGE_ALT = 'Mesh social preview card reading “Built for distributed systems” with one public app URL, server-side runtime placement, and operator truth on meshc cluster.'
const SITE_LOGO_URL = `${SITE_URL}/logo-icon-black.svg`
const INDEX_ROBOTS = 'index,follow,max-image-preview:large,max-snippet:-1,max-video-preview:-1'
const NOINDEX_ROBOTS = 'noindex,follow,max-image-preview:large,max-snippet:-1,max-video-preview:-1'

function toCanonicalUrl(relativePath: string): string {
  if (!relativePath || relativePath === 'index.md') {
    return `${SITE_URL}/`
  }

  const cleanPath = relativePath
    .replace(/index\.md$/, '')
    .replace(/\.md$/, '')

  return `${SITE_URL}/${cleanPath}`
}

function pageTitle(pageData: PageData): string {
  if (pageData.relativePath === 'index.md') {
    return SITE_NAME
  }

  return pageData.title ? `${pageData.title} | Mesh` : SITE_NAME
}

function pageDescription(pageData: PageData): string {
  if (pageData.description) {
    return pageData.description
  }

  if (pageData.relativePath === 'index.md') {
    return DEFAULT_DESCRIPTION
  }

  return 'Documentation for Mesh, the programming language built for distributed systems, typed concurrency, and native backends.'
}

function pageOgType(relativePath: string): 'website' | 'article' {
  return relativePath.startsWith('docs/') ? 'article' : 'website'
}

function pageRobots(relativePath: string): string {
  return relativePath === 'packages/package.md' ? NOINDEX_ROBOTS : INDEX_ROBOTS
}

function pageStructuredData(pageData: PageData, title: string, description: string, canonicalUrl: string): string {
  const relativePath = pageData.relativePath
  const isHome = relativePath === 'index.md'
  const isDocsPage = relativePath.startsWith('docs/')
  const isPackagesIndex = relativePath === 'packages/index.md'

  let pageType = 'WebPage'
  if (isDocsPage) {
    pageType = 'TechArticle'
  } else if (isPackagesIndex) {
    pageType = 'CollectionPage'
  }

  const pageNode: Record<string, unknown> = {
    '@type': pageType,
    '@id': `${canonicalUrl}#page`,
    url: canonicalUrl,
    name: title,
    description,
    inLanguage: 'en-US',
    isPartOf: { '@id': `${SITE_URL}/#website` },
    publisher: { '@id': `${SITE_URL}/#organization` },
    image: { '@id': `${SOCIAL_IMAGE_URL}#image` },
    primaryImageOfPage: { '@id': `${SOCIAL_IMAGE_URL}#image` },
  }

  if (isDocsPage) {
    pageNode.headline = title
    pageNode.mainEntityOfPage = canonicalUrl
  }

  if (isHome) {
    pageNode.about = [
      'distributed systems',
      'programming language',
      'actors',
      'fault tolerance',
      'native compilation',
    ]
  }

  return JSON.stringify({
    '@context': 'https://schema.org',
    '@graph': [
      {
        '@type': 'Organization',
        '@id': `${SITE_URL}/#organization`,
        name: SITE_NAME,
        url: SITE_URL,
        logo: {
          '@type': 'ImageObject',
          url: SITE_LOGO_URL,
        },
        sameAs: ['https://github.com/hyperpush-org/mesh-lang'],
      },
      {
        '@type': 'WebSite',
        '@id': `${SITE_URL}/#website`,
        url: SITE_URL,
        name: SITE_NAME,
        description: DEFAULT_DESCRIPTION,
        inLanguage: 'en-US',
        publisher: { '@id': `${SITE_URL}/#organization` },
        image: { '@id': `${SOCIAL_IMAGE_URL}#image` },
      },
      {
        '@type': 'ImageObject',
        '@id': `${SOCIAL_IMAGE_URL}#image`,
        url: SOCIAL_IMAGE_URL,
        width: 1200,
        height: 630,
        caption: SOCIAL_IMAGE_ALT,
      },
      pageNode,
    ],
  })
}

export default defineConfig({
  title: 'Mesh',
  titleTemplate: ':title | Mesh',
  description: DEFAULT_DESCRIPTION,

  // Respect system preference by default; user can override via toggle
  appearance: 'auto',

  // Enable clean URLs
  cleanUrls: true,

  // Enable git-based last-updated timestamps
  lastUpdated: true,

  // Generate sitemap
  sitemap: {
    hostname: SITE_URL,
  },

  // Site-wide SEO defaults
  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/logo-icon-black.svg', media: '(prefers-color-scheme: light)' }],
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/logo-icon-white.svg', media: '(prefers-color-scheme: dark)' }],
    ['link', { rel: 'image_src', href: SOCIAL_IMAGE_URL }],
    ['meta', { name: 'theme-color', content: '#ffffff', media: '(prefers-color-scheme: light)' }],
    ['meta', { name: 'theme-color', content: '#0d0d0d', media: '(prefers-color-scheme: dark)' }],
    ['meta', { property: 'og:site_name', content: SITE_NAME }],
    ['meta', { property: 'og:locale', content: 'en_US' }],
    ['meta', { property: 'og:image', content: SOCIAL_IMAGE_URL }],
    ['meta', { property: 'og:image:url', content: SOCIAL_IMAGE_URL }],
    ['meta', { property: 'og:image:secure_url', content: SOCIAL_IMAGE_URL }],
    ['meta', { property: 'og:image:type', content: 'image/png' }],
    ['meta', { property: 'og:image:width', content: '1200' }],
    ['meta', { property: 'og:image:height', content: '630' }],
    ['meta', { property: 'og:image:alt', content: SOCIAL_IMAGE_ALT }],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:image', content: SOCIAL_IMAGE_URL }],
    ['meta', { name: 'twitter:image:src', content: SOCIAL_IMAGE_URL }],
    ['meta', { name: 'twitter:image:alt', content: SOCIAL_IMAGE_ALT }],
    ['meta', { name: 'twitter:site', content: '@meshlang' }],
  ],

  // Per-page dynamic SEO meta tags
  transformPageData(pageData) {
    const canonicalUrl = toCanonicalUrl(pageData.relativePath)
    const title = pageTitle(pageData)
    const description = pageDescription(pageData)
    const robots = pageRobots(pageData.relativePath)
    const pageHead: HeadConfig[] = [
      ['link', { rel: 'canonical', href: canonicalUrl }],
      ['meta', { name: 'description', content: description }],
      ['meta', { name: 'robots', content: robots }],
      ['meta', { name: 'googlebot', content: robots }],
      ['meta', { property: 'og:title', content: title }],
      ['meta', { property: 'og:description', content: description }],
      ['meta', { property: 'og:url', content: canonicalUrl }],
      ['meta', { property: 'og:type', content: pageOgType(pageData.relativePath) }],
      ['meta', { name: 'twitter:title', content: title }],
      ['meta', { name: 'twitter:description', content: description }],
      ['script', { type: 'application/ld+json' }, pageStructuredData(pageData, title, description, canonicalUrl)],
    ]

    pageData.frontmatter.head ??= []
    pageData.frontmatter.head.push(...pageHead)
  },

  markdown: {
    languages: [
      {
        ...(meshGrammar as any),
        name: 'mesh',
      },
    ],
    theme: {
      light: meshLight as any,
      dark: meshDark as any,
    },
  },

  themeConfig: {
    nav: [
      { text: 'Docs', link: '/docs/' },
      { text: 'Packages', link: 'https://packages.meshlang.dev', target: '_blank' },
    ],
    search: { provider: 'local' },
    editLink: {
      pattern: 'https://github.com/hyperpush-org/mesh-lang/edit/main/website/docs/:path',
      text: 'Edit this page on GitHub',
    },
    meshVersion: '14.0',
    sidebar: {
      '/docs/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Introduction', link: '/docs/getting-started/', icon: 'BookOpen' } as any,
            { text: 'Clustered Example', link: '/docs/getting-started/clustered-example/', icon: 'Network' } as any,
          ],
        },
        {
          text: 'Language Guide',
          collapsed: false,
          items: [
            { text: 'Language Basics', link: '/docs/language-basics/', icon: 'Code2' } as any,
            { text: 'Type System', link: '/docs/type-system/', icon: 'Shapes' } as any,
            { text: 'Iterators', link: '/docs/iterators/', icon: 'Repeat' } as any,
            { text: 'Concurrency', link: '/docs/concurrency/', icon: 'Workflow' } as any,
          ],
        },
        {
          text: 'Web & Networking',
          collapsed: false,
          items: [
            { text: 'Web', link: '/docs/web/', icon: 'Globe' } as any,
          ],
        },
        {
          text: 'Data',
          collapsed: false,
          items: [
            { text: 'Databases', link: '/docs/databases/', icon: 'Database' } as any,
          ],
        },
        {
          text: 'Distribution',
          collapsed: false,
          items: [
            { text: 'Distributed Actors', link: '/docs/distributed/', icon: 'Network' } as any,
          ],
        },
        {
          text: 'Tooling',
          collapsed: false,
          items: [
            { text: 'Developer Tools', link: '/docs/tooling/', icon: 'Wrench' } as any,
          ],
        },
        {
          text: 'Standard Library',
          collapsed: false,
          items: [
            { text: 'Standard Library', link: '/docs/stdlib/', icon: 'Library' } as any,
            { text: 'Testing', link: '/docs/testing/', icon: 'FlaskConical' } as any,
          ],
        },
        {
          text: 'Reference',
          collapsed: false,
          items: [
            { text: 'Syntax Cheatsheet', link: '/docs/cheatsheet/', icon: 'ClipboardList' } as any,
          ],
        },
        {
          text: 'Proof Surfaces',
          collapsed: false,
          items: [
            {
              text: 'Distributed Proof',
              link: '/docs/distributed-proof/',
              icon: 'ShieldCheck',
              includeInFooter: false,
            } as any,
            {
              text: 'Production Backend Proof',
              link: '/docs/production-backend-proof/',
              icon: 'ShieldCheck',
              includeInFooter: false,
            } as any,
          ],
        },
      ],
    },
    outline: { level: [2, 3], label: 'On this page' },
  },

  vite: {
    plugins: [
      tailwindcss(),
    ],
    resolve: {
      alias: {
        '@': path.resolve(__dirname, './theme'),
      },
    },
  },
})
