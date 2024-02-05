export const routes = [
  {
    path: '/',
    name: 'proxy',
    component: () => import('@/views/proxy/ProxyView.vue'),
  },
  {
    path: '/setting',
    name: 'setting',
    component: () => import('@/views/setting/SettingView.vue'),
  },
  {
    path: '/rule',
    name: 'rule',
    component: () => import('@/views/rule/RuleView.vue'),
  },
]
