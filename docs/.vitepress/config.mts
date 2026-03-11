import { defineConfig } from 'vitepress'
import ilkLang from './langs/ilk.tmLanguage.json'
import kliLang from './langs/kli.tmLanguage.json'

export default defineConfig({
  title: 'ilk/kli',
  description: 'Two-level data modeling with provenance tracking',
  base: '/ilk-lang/',

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'ilk Spec', link: '/ilk-spec' },
      { text: 'kli Spec', link: '/kli-spec' }
    ],

    sidebar: [
      {
        text: 'Documentation',
        items: [
          { text: 'Introduction', link: '/' },
          { text: 'ilk Schema Language', link: '/ilk-spec' },
          { text: 'kli Domain Model', link: '/kli-spec' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/err0r500/ilk-lang' }
    ]
  },

  markdown: {
    languages: [ilkLang as any, kliLang as any]
  }
})
