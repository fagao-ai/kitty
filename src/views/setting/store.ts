import { useLocalStorage } from '@vueuse/core'

const settingStore = useLocalStorage('setting', {
  autoUpdate: 3,
})

export { settingStore }
