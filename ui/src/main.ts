import '@/global.css'
import 'floating-vue/dist/style.css'

import { createApp } from 'vue'
import FloatingVue from 'floating-vue'

import App from './App.vue'

import Client from '@/lib/comet'
import { url } from './utils'

import { toast, type Id } from 'vue3-toastify'

// These are states for the UI, serves no purpose whatsoever
// but provide good UX to the user.
const uiState: {
  initiallyConnected: boolean
  toastId: Id | null
} = {
  initiallyConnected: false,
  toastId: null
}

const client = new Client(
  import.meta.env.PROD
    ? url('/api/ws/pineapple')
    : // i run the backend as a standalone server while in dev mode
      'ws://uwuntu:3000/api/ws/pineapple'
)

client.onopen = () => {
  if (uiState.initiallyConnected) {
    if (uiState.toastId) toast.remove(uiState.toastId)

    return toast('Reconnected to server', {
      type: 'success',
      position: 'bottom-right',
      autoClose: 2000,
      hideProgressBar: true,
      transition: 'slide'
    })
  }

  uiState.initiallyConnected = !uiState.initiallyConnected
}

client.onclose = () => {
  uiState.toastId = toast('Lost connection to server, reconnecting...', {
    type: 'error',
    isLoading: true,
    position: 'bottom-right',
    autoClose: false,
    hideProgressBar: true,
    transition: 'slide'
  })

  client.reconnect()
}

const app = createApp(App)
app.provide('client', client)

// Using floating-vue only because it handles alot of edge cases,
// and easier to work with in general.
app.use(FloatingVue)

app.mount('#app')
