import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Comet',
      component: () => import("../views/CometView.vue")
    },
    {
      path: '/settings',
      name: 'Settings',
      component: () => import("../views/SettingsView.vue")
    },
  ]
})

export default router;
