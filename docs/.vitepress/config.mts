import { defineConfig } from 'vitepress'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import ilkLang from './langs/ilk.tmLanguage.json'

export default defineConfig({
    title: 'ilk',
    description: 'Single-file data modeling with provenance tracking',
    base: '/ilk-lang/',

    vite: {
        plugins: [
            wasm(),
            topLevelAwait()
        ]
    },


    themeConfig: {
        nav: [
            { text: 'Home', link: '/' },
            { text: 'Examples', link: '/examples' },
            { text: 'Playground', link: '/playground' },
            { text: 'Specification', link: '/ilk-spec' }
        ],

        socialLinks: [
            { icon: 'github', link: 'https://github.com/err0r500/ilk-lang' }
        ]
    },

    markdown: {
        languages: [ilkLang as any]
    }
})
