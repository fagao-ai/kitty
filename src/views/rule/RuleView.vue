<script
  setup
  lang="ts"
>
import { reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { NForm, NFormItem } from 'naive-ui'
import type { ProxyRule } from '@/types/rule'
import { createRule, deleteRule, getAllRules, updateRule } from '@/apis/rule'
import HeaderBar from '@/components/HeaderBar.vue'

const { t } = useI18n()

interface RulesForm {
  rules: ProxyRule[]
}

const defaultRule: ProxyRule = {
  id: 0,
  ruleAction: 'proxy',
  ruleType: 'full_domain',
  rule: '',
}

const defaultRulesFrom: RulesForm = {
  rules: [{ ...defaultRule }],
}
const rulesForm = reactive<RulesForm>(defaultRulesFrom)

function handleAddRule() {
  rulesForm.rules.push({ ...defaultRule })
}

async function handleRemoveRule(index: number) {
  const id = rulesForm.rules[index].id
  if (id)
    await deleteRule(id)

  rulesForm.rules.splice(index, 1)

  if (rulesForm.rules.length === 0)
    rulesForm.rules.push({ ...defaultRule })
}

async function handleUpdateRule(rule: ProxyRule) {
  if (!rule.rule)
    return

  if (!rule.id) {
    await createRule(rule)
    return
  }
  await updateRule({ id: rule.id, rule: rule.rule, ruleAction: rule.ruleAction, ruleType: rule.ruleType })
}

async function initRules() {
  const rules = await getAllRules()
  Object.assign(rulesForm.rules, rules.length === 0 ? defaultRulesFrom : rules)
}
initRules()
</script>

<template>
  <div class="flex w-full h-full flex-col">
    <header-bar>
      <template #title>
        {{ t('menubar.rules') }}
      </template>
    </header-bar>
    <div class="flex-1 overflow-y-auto pr-4">
      <n-scrollbar style="max-height: 100%;">
        <n-form
          :model="rulesForm"
          size="medium"
          label-placement="left"
          label-width="auto"
        >
          <n-form-item
            v-for="(item, index) in rulesForm.rules"
            :key="index"
            :path="`rulesForm.rules[${index}]`"
          >
            <div class="flex gap-x-4 w-full">
              <n-select
                v-model:value="item.ruleAction"
                :options="[{ label: 'PROXY', value: 'proxy' }, { label: 'DIRECT', value: 'direct' }, { label: 'REJECT', value: 'reject' }]"
                @blur="handleUpdateRule(item)"
              />
              <n-select
                v-model:value="item.ruleType"
                :options="[{ label: 'DOMAIN SUFFIX', value: 'domain_suffix' }, { label: 'DOMAIN PREFFIX', value: 'domain_preffix' }, { label: 'FULL DOMAIN', value: 'full_domain' }, { label: 'CIDR', value: 'cidr' }]"
                @blur="handleUpdateRule(item)"
              />
              <n-input
                v-model:value="item.rule"
                @blur="handleUpdateRule(item)"
              />
              <n-button
                class="pl-3"
                @click="handleAddRule"
              >
                +
              </n-button>
              <n-button
                class="pl-3"
                @click="handleRemoveRule(index)"
              >
                -
              </n-button>
            </div>
          </n-form-item>
        </n-form>
      </n-scrollbar>
    </div>
  </div>
</template>
