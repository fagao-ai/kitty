import { camelizeKeys, decamelizeKeys } from 'humps'
import type { ProxyRule } from '@/types/rule'
import { invoke } from '@/utils/invoke'

export async function updateRule(rule: ProxyRule) {
  await invoke('update_rules_item', { records: [decamelizeKeys(rule)] })
}

export async function getAllRules() {
  const res = await invoke<ProxyRule[]>('query_rules')
  return camelizeKeys(res.data) as ProxyRule[]
}

export async function createRule(rule: ProxyRule) {
  await invoke('add_rules', { records: [decamelizeKeys(rule)] })
}

export async function deleteRule(id: number) {
  await invoke('delete_rules', { ids: [id] })
}
