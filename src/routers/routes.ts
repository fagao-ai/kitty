export const routes = [
  {
    path: '/',
    name: 'proxy',
    component: async () => await import('@/views/proxy/ProxyView.vue'),
  },
]
