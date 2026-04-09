import type { Theme } from 'vitepress'
import Layout from './Layout.vue'
import './styles/main.css'

export default {
  Layout,
  enhanceApp({ app }) {
    // Register global components if needed
  },
} satisfies Theme
