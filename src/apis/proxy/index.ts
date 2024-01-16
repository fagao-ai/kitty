import { invoke } from '@/utils/invoke'
import type { ProxyCard as Card } from '@/types/proxy'

export async function batchGetProxy() {
  const res = await invoke<Card[]>('get_all_proxies')
  return res.data.map(item => ({
    tag: 'hysteria',
    name: item.name,
    delay: 200,
    protocol: 'TCP',
  }))
  // cards.value.push({
  //   tag: 'hysteria',
  //   name: 'test',
  //   delay: 200,
  //   protocol: 'TCP',
  // })
}
