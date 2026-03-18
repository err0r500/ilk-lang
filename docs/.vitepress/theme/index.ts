import DefaultTheme from 'vitepress/theme'
import Playground from '../components/Playground.vue'
import TypeExample from '../components/TypeExample.vue'
import './style.css'

export default {
  extends: DefaultTheme,
  enhanceApp({ app }: { app: any }) {
    app.component('Playground', Playground)
    app.component('TypeExample', TypeExample)
  }
}
