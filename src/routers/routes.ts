export const routes = [
  {
    path: '/',
    name: 'proxy',
    component: () => import('@/views/proxy/ProxyView.vue'),
  },
  {
    path: '/subscription',
    name: 'subscription',
    component: () => import('@/views/subscription/SubscriptionView.vue'),
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
  {
    path: '/log',
    name: 'log',
    component: () => import('@/views/log/LogView.vue'),
  },
]
