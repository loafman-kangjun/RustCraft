import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Document for RustCraft Game",
  description: "RustCraft (Minecraft in Rust)",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: './' },
      { text: 'Examples', link: './markdown-examples' }
    ],

    sidebar: [
      {
        text: 'Examples',
        items: [
          { text: 'Markdown Examples', link: './markdown-examples' },
          { text: 'Runtime API Examples', link: './api-examples' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/loafman-kangjun/RustCraft' }
    ],

    i18nRouting: true,
  },

  locales: {
    zh: {
      label: '中文',
      lang: 'zh-CN',
    },
    en: {
      label: 'English',
      lang: 'en-US',
    },
  }
})
