<script
  setup
  lang="ts"
>
import { reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { useToast } from 'primevue/usetoast'
import CIDR from 'ip-cidr'
import type { ProxyRule } from '@/types/rule'
import { createRule, deleteRule, getAllRules, updateRule } from '@/apis/rule'
import HeaderBar from '@/components/HeaderBar.vue'
import Button from 'primevue/button'
import Select from 'primevue/select'
import InputText from 'primevue/inputtext'
import ScrollPanel from 'primevue/scrollpanel'

const { t } = useI18n()
const toast = useToast()

interface RulesForm {
  rules: ProxyRule[]
}

const defaultRule: ProxyRule = {
  id: 0,
  ruleAction: 'direct',
  ruleType: 'domain_suffix',
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
  if (id) {
    await deleteRule(id)
    toast.add({ severity: 'success', summary: 'Success', detail: t('common.deleteSuccess'), life: 3000 })
  }

  rulesForm.rules.splice(index, 1)

  if (rulesForm.rules.length === 0)
    rulesForm.rules.push({ ...defaultRule })
}

async function handleUpdateRule(rule: ProxyRule) {
  if (!rule.rule)
    return

  if (rule.ruleType === 'cidr' && !CIDR.isValidCIDR(rule.rule)) {
    toast.add({ severity: 'error', summary: 'Error', detail: t('rule.invalidCIDR'), life: 5000 })
    return
  }

  if (!rule.id) {
    await createRule(rule)
    toast.add({ severity: 'success', summary: 'Success', detail: t('common.createSuccess'), life: 3000 })
    return
  }
  await updateRule({ id: rule.id, rule: rule.rule, ruleAction: rule.ruleAction, ruleType: rule.ruleType })
  toast.add({ severity: 'success', summary: 'Success', detail: t('common.createSuccess'), life: 3000 })
}

async function initRules() {
  const rules = await getAllRules()
  Object.assign(rulesForm.rules, rules.length === 0 ? defaultRulesFrom : rules)
}
initRules()

const ruleActionOptions = [
  { label: 'DIRECT', value: 'direct' },
  { label: 'PROXY', value: 'proxy' },
  { label: 'REJECT', value: 'reject' }
]

const ruleTypeOptions = [
  { label: 'DOMAIN SUFFIX', value: 'domain_suffix' },
  { label: 'DOMAIN PREFFIX', value: 'domain_preffix' },
  { label: 'FULL DOMAIN', value: 'full_domain' },
  { label: 'CIDR', value: 'cidr' }
]
</script>

<template>
  <div class="flex w-full h-full flex-col">
    <header-bar>
      <template #title>
        {{ t('menubar.rules') }}
      </template>
      <template #default>
        <Button
          text
          class="text-4xl !p-0"
          @click="handleAddRule"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            class="w-6 h-6"
          >
            <path
              d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2zm4 11h-3v3c0 .55-.45 1-1 1s-1-.45-1-1v-3H8c-.55 0-1-.45-1-1s.45-1 1-1h3V8c0-.55.45-1 1-1s1 .45 1 1v3h3c.55 0 1 .45 1 1s-.45 1-1 1z"
              fill="currentColor"
            />
          </svg>
        </Button>
      </template>
    </header-bar>
    <div class="flex-1 overflow-y-auto pr-4">
      <ScrollPanel style="max-height: 100%;">
        <div class="flex flex-col gap-4">
          <div
            v-for="(item, index) in rulesForm.rules"
            :key="index"
            class="flex gap-4 w-full items-center"
          >
            <Select
              v-model="item.ruleAction"
              :options="ruleActionOptions"
              option-label="label"
              option-value="value"
              class="w-40"
              @change="handleUpdateRule(item)"
            />
            <Select
              v-model="item.ruleType"
              :options="ruleTypeOptions"
              option-label="label"
              option-value="value"
              class="w-48"
              @change="handleUpdateRule(item)"
            />
            <InputText
              v-model="item.rule"
              class="flex-1"
              @blur="handleUpdateRule(item)"
            />
            <Button
              text
              class="text-2xl !p-0"
              @click="handleRemoveRule(index)"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 1024 1024"
                class="w-5 h-5"
              >
                <path
                  d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448s448-200.6 448-448S759.4 64 512 64zm192 472c0 4.4-3.6 8-8 8H328c-4.4 0-8-3.6-8-8v-48c0-4.4 3.6-8 8-8h368c4.4 0 8 3.6 8 8v48z"
                  fill="currentColor"
                />
              </svg>
            </Button>
          </div>
        </div>
      </ScrollPanel>
    </div>
  </div>
</template>
