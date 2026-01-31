<script
  setup
  lang="ts"
>
import { reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { NForm, NFormItem, useMessage, NButton } from 'naive-ui'
import CIDR from 'ip-cidr'
import type { ProxyRule } from '@/types/rule'
import { createRule, deleteRule, getAllRules, updateRule, exportRules, importRules } from '@/apis/rule'
import { save, open } from '@tauri-apps/plugin-dialog'
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'
import HeaderBar from '@/components/HeaderBar.vue'

const { t } = useI18n()
const message = useMessage()

interface RulesForm {
  rules: ProxyRule[]
}

const defaultRule: ProxyRule = {
  id: 0,
  action: 'direct',
  ruleType: 'domain_suffix',
  pattern: '',
}

const defaultRulesFrom: RulesForm = {
  rules: [{ ...defaultRule }],
}
const rulesForm = reactive<RulesForm>(defaultRulesFrom)

function handleAddRule() {
  rulesForm.rules.push({ ...defaultRule })
}

async function handleRemoveRule(index: number) {
  // Backend expects 1-based index
  await deleteRule(index + 1)
  message.success(t('common.deleteSuccess'))

  rulesForm.rules.splice(index, 1)

  if (rulesForm.rules.length === 0)
    rulesForm.rules.push({ ...defaultRule })
}

async function handleUpdateRule(rule: ProxyRule) {
  if (!rule.pattern)
    return

  if (rule.ruleType === 'cidr' && !CIDR.isValidCIDR(rule.pattern)) {
    message.error(t('rule.invalidCIDR'), { duration: 5000 })
    return
  }

  // Save all rules to file (replaces entire file)
  await updateRule(rulesForm.rules)
  message.success(t('common.createSuccess'))
}

async function initRules() {
  const rules = await getAllRules()
  Object.assign(rulesForm.rules, rules.length === 0 ? defaultRulesFrom : rules)
}

async function handleExport() {
  try {
    const jsonContent = await exportRules()

    const filePath = await save({
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }],
      defaultPath: 'custom_rules.json'
    })

    if (filePath) {
      await writeTextFile(filePath, jsonContent)
      message.success(t('rule.exportSuccess'))
    }
  }
  catch (e) {
    message.error(`${t('rule.exportFailed')}: ${e}`)
    console.error('Export error:', e)
  }
}

async function handleImport() {
  try {
    const filePath = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    })

    if (filePath) {
      const content = await readTextFile(filePath)
      await importRules(content)
      message.success(t('rule.importSuccess'))
      // Reload rules after import
      await initRules()
    }
  }
  catch (e) {
    message.error(`${t('rule.importFailed')}: ${e}`)
    console.error('Import error:', e)
  }
}

initRules()
</script>

<template>
  <div class="flex w-full h-full flex-col">
    <header-bar @toggle-mobile-menu="$emit('toggle-mobile-menu')">
      <template #mobile-menu-button>
        <n-icon size="24">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 12h18M3 6h18M3 18h18" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </n-icon>
      </template>
      <template #title>
        {{ t('menubar.rules') }}
      </template>
      <template #mobile-actions>
        <n-button
          size="small"
          @click="handleAddRule"
        >
          <n-icon>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 5v14M5 12h14" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </n-icon>
        </n-button>
        <n-button
          size="small"
          @click="handleImport"
        >
          <n-icon>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" stroke-linejoin="round"/>
              <polyline points="7 10 12 15 17 10" stroke-linecap="round" stroke-linejoin="round"/>
              <line x1="12" y1="15" x2="12" y2="3" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </n-icon>
        </n-button>
        <n-button
          size="small"
          @click="handleExport"
        >
          <n-icon>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" stroke-linejoin="round"/>
              <polyline points="17 8 12 3 7 8" stroke-linecap="round" stroke-linejoin="round"/>
              <line x1="12" y1="3" x2="12" y2="15" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </n-icon>
        </n-button>
      </template>
      <template #default>
        <n-button
          text
          class="text-4xl"
          @click="handleAddRule"
        >
          <n-icon>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              viewBox="0 0 24 24"
            >
              <path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2zm4 11h-3v3c0 .55-.45 1-1 1s-1-.45-1-1v-3H8c-.55 0-1-.45-1-1s.45-1 1-1h3V8c0-.55.45-1 1-1s1 .45 1 1v3h3c.55 0 1 .45 1 1s-.45 1-1 1z"
                fill="currentColor"
              />
            </svg>
          </n-icon>
        </n-button>
        <n-button
          text
          class="text-4xl"
          @click="handleImport"
          :title="t('common.import')"
        >
          <n-icon>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" stroke-linejoin="round"/>
              <polyline points="7 10 12 15 17 10" stroke-linecap="round" stroke-linejoin="round"/>
              <line x1="12" y1="15" x2="12" y2="3" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </n-icon>
        </n-button>
        <n-button
          text
          class="text-4xl"
          @click="handleExport"
          :title="t('rule.export')"
        >
          <n-icon>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" stroke-linejoin="round"/>
              <polyline points="17 8 12 3 7 8" stroke-linecap="round" stroke-linejoin="round"/>
              <line x1="12" y1="3" x2="12" y2="15" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </n-icon>
        </n-button>
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
                v-model:value="item.action"
                :options="[{ label: 'DIRECT', value: 'direct' }, { label: 'PROXY', value: 'proxy' }, { label: 'REJECT', value: 'reject' }]"
                @blur="handleUpdateRule(item)"
              />
              <n-select
                v-model:value="item.ruleType"
                :options="[{ label: 'DOMAIN SUFFIX', value: 'domain_suffix' }, { label: 'DOMAIN PREFFIX', value: 'domain_preffix' }, { label: 'FULL DOMAIN', value: 'full_domain' }, { label: 'CIDR', value: 'cidr' }]"
                @blur="handleUpdateRule(item)"
              />
              <n-input
                v-model:value="item.pattern"
                @blur="handleUpdateRule(item)"
              />
              <n-button
                class="text-2xl"
                text
                @click="handleRemoveRule(index)"
              >
                <n-icon>
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                    viewBox="0 0 1024 1024"
                  >
                    <path
                      d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448s448-200.6 448-448S759.4 64 512 64zm192 472c0 4.4-3.6 8-8 8H328c-4.4 0-8-3.6-8-8v-48c0-4.4 3.6-8 8-8h368c4.4 0 8 3.6 8 8v48z"
                      fill="currentColor"
                    />
                  </svg>
                </n-icon>
              </n-button>
            </div>
          </n-form-item>
        </n-form>
      </n-scrollbar>
    </div>
  </div>
</template>
