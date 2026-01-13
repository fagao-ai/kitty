import type { Ref } from 'vue'
import { useLocalStorage } from '@vueuse/core'
import { ProxyType } from '@/types/proxy'

interface ProxyStore {
  currentProxy: ProxyType
}

const proxyStore = useLocalStorage('proxy', {
  currentProxy: ProxyType.Hysteria,
}) as Ref<ProxyStore>

export { proxyStore }
