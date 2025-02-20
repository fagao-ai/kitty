import { useLocalStorage } from '@vueuse/core'

const settingStore = useLocalStorage('setting', {
  autoUpdate: 3,
  sysproxyFlag: false,
  port: 11080,
})

export { settingStore }
