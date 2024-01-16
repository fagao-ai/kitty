import { useLocalStorage } from '@vueuse/core'
import { ProxyType } from '@/types/proxy'

const proxyStore = useLocalStorage('proxy', {
  currentProxy: ProxyType.Hysteria,
})

export { proxyStore }
