import { useLocalStorage } from '@vueuse/core'

const settingStore = useLocalStorage('setting', {
  language: 'zh-CN',
  autoUpdate: 3,
})

export { settingStore }
