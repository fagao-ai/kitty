import { createRouter, createWebHashHistory } from 'vue-router'
import { routes } from '@/routers/routes'

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
