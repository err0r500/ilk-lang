import { defineConfig } from 'vitepress'
import ilkLang from './langs/ilk.tmLanguage.json'

export default defineConfig({
  title: 'ilk',
  description: 'Single-file data modeling with provenance tracking',
  base: '/ilk-lang/',

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Specification', link: '/ilk-spec' }
    ],

    sidebar: [
      {
        text: 'Documentation',
        items: [
          { text: 'Introduction', link: '/' },
          { text: 'Language Specification', link: '/ilk-spec' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/err0r500/ilk-lang' }
    ]
  },

  markdown: {
    languages: [ilkLang as any]
  }
})
