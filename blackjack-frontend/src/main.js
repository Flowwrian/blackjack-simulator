import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import Logo from './components/Logo.vue'

const app = createApp(App)
// eslint-disable-next-line vue/multi-word-component-names
app.component("Logo", Logo)

app.use(createPinia())

app.mount('#app')
