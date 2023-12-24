import { useLocalStorage } from '@vueuse/core'

const settingStore = useLocalStorage('setting', {
  language: 'zh-CN',
})

export { settingStore }
