import { useLocalStorage } from '@vueuse/core'
import { ProxyType } from '@/types/proxy'

const proxyStore = useLocalStorage('proxy', {
  currentProxy: ProxyType.Hysteria,  // Legacy, kept for compatibility
  activeProxyId: null as number | null,      // Currently active proxy ID
  activeProxyType: null as ProxyType | null,  // Currently active proxy type
})

export { proxyStore }
