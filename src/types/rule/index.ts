export type RuleAction = 'proxy' | 'direct' | 'reject'

export type RuleType = 'domain_suffix' | 'domain_preffix' | 'full_domain' | 'cidr'

export interface ProxyRule {
  id?: number
  ruleAction: RuleAction
  ruleType: RuleType
  rule: string
}
