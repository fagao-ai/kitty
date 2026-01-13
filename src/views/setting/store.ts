import { useLocalStorage } from '@vueuse/core'
import type { Ref } from 'vue'

interface SettingStore {
  autoUpdate: number
  sysproxyFlag: boolean
  port: number
}

const settingStore = useLocalStorage('setting', {
  autoUpdate: 3,
  sysproxyFlag: false,
  port: 11080,
}) as Ref<SettingStore>

export { settingStore }
