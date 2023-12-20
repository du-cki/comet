import '@/global.css'
import 'floating-vue/dist/style.css'

import { createApp } from 'vue'
import FloatingVue from 'floating-vue'

import App from './App.vue'
import router from './router'

import Client from '@/lib/comet'
import { url } from './utils'

const app = createApp(App)
app.provide('client', new Client(
    import.meta.env.PROD
        ? url('/api/ws/pineapple')
        // i run the backend as a standalone server while in dev mode
        : 'ws://100.127.105.135:3000/api/ws/pineapple'
))


app.use(router)
// Using floating-vue only because it handles alot of edge cases,
// and easier to work with in general.
app.use(FloatingVue)

app.mount('#app')
