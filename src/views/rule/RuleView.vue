<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NIcon, NInput, NSelect, NTag, useMessage } from 'naive-ui'
import { VueDraggable } from 'vue-draggable-plus'
import CIDR from 'ip-cidr'
import { createRule, deleteRule, getAllRules, updateRule, exportRules, importRules } from '@/apis/rule'
import { save, open } from '@tauri-apps/plugin-dialog'
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'
import HeaderBar from '@/components/HeaderBar.vue'

defineEmits<{
  toggleMobileMenu: []
}>()

const { t } = useI18n()
const message = useMessage()

interface RuleWithId {
  uuid: string
  action: string
  ruleType: string
  pattern: string
}

const rules = ref<RuleWithId[]>([])
const editingRuleId = ref<string | null>(null)
const isLoading = ref(false)

// Action options with color coding
const actionOptions = computed(() => [
  { label: 'DIRECT', value: 'direct', color: '#18a058' },
  { label: 'PROXY', value: 'proxy', color: '#5352ed' },
  { label: 'REJECT', value: 'reject', color: '#d03050' },
])

// Rule type options
const ruleTypeOptions = computed(() => [
  { label: 'DOMAIN SUFFIX', value: 'domain_suffix' },
  { label: 'DOMAIN PREFIX', value: 'domain_preffix' },
  { label: 'FULL DOMAIN', value: 'full_domain' },
  { label: 'CIDR', value: 'cidr' },
])

// Get action color for UI
function getActionColor(action: string): string {
  return actionOptions.value.find(opt => opt.value === action)?.color || '#999'
}

// Get action icon
function getActionIcon(action: string) {
  const icons = {
    direct: 'M13 2L3 14h9l-1 8 10-12h-9l1-8z',
    proxy: 'M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z',
    reject: 'M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z',
  }
  return icons[action as keyof typeof icons] || icons.direct
}

// Generate unique ID
function generateUUID(): string {
  return `${Date.now()}-${Math.random().toString(36).substring(2, 9)}`
}

// Handle add rule
function handleAddRule() {
  const newRule: RuleWithId = {
    uuid: generateUUID(),
    action: 'direct',
    ruleType: 'domain_suffix',
    pattern: '',
  }
  rules.value.push(newRule)
  editingRuleId.value = newRule.uuid
}

// Handle remove rule
async function handleRemoveRule(uuid: string) {
  const index = rules.value.findIndex(r => r.uuid === uuid)
  if (index === -1)
    return

  // Backend uses 1-based index
  try {
    await deleteRule(index + 1)
    message.success(t('common.deleteSuccess'))
  }
  catch (e: any) {
    message.error(`Failed to delete: ${e?.message || 'Unknown error'}`)
    return
  }

  rules.value.splice(index, 1)
  // Don't auto-add new rule, let user click Add button
}

// Handle update rule
async function handleUpdateRule(rule: RuleWithId) {
  if (!rule.pattern?.trim()) {
    message.warning('Pattern cannot be empty')
    return
  }

  if (rule.ruleType === 'cidr' && !CIDR.isValidCIDR(rule.pattern)) {
    message.error(t('rule.invalidCIDR'), { duration: 5000 })
    return
  }

  isLoading.value = true
  try {
    await updateRule(rules.value.map(r => ({ action: r.action, ruleType: r.ruleType, pattern: r.pattern })))
    message.success(t('common.updateSuccess'))
    editingRuleId.value = null
  }
  catch (e: any) {
    message.error(`Failed to update: ${e?.message || 'Unknown error'}`)
  }
  finally {
    isLoading.value = false
  }
}

// Handle export
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
  catch (e: any) {
    message.error(`${t('rule.exportFailed')}: ${e}`)
  }
}

// Handle import
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
      await initRules()
    }
  }
  catch (e: any) {
    message.error(`${t('rule.importFailed')}: ${e}`)
  }
}

// Handle drag end
async function handleDragEnd() {
  // Save new order
  if (rules.value.length > 0) {
    await updateRule(rules.value.map(r => ({ action: r.action, ruleType: r.ruleType, pattern: r.pattern })))
    message.success(t('common.updateSuccess'))
  }
}

// Initialize rules
async function initRules() {
  const fetchedRules = await getAllRules()
  rules.value = fetchedRules.map(r => ({ action: r.action, ruleType: r.ruleType, pattern: r.pattern, uuid: generateUUID() }))
}

// Start editing
function startEditing(uuid: string) {
  editingRuleId.value = uuid
}

// Cancel editing
function cancelEditing(uuid: string) {
  const rule = rules.value.find(r => r.uuid === uuid)
  if (rule && !rule.pattern) {
    // Remove empty rule on cancel (only from local, not calling API)
    const index = rules.value.findIndex(r => r.uuid === uuid)
    if (index !== -1) {
      rules.value.splice(index, 1)
    }
  }
  editingRuleId.value = null
}

// Load rules on mount
initRules()
</script>

<template>
  <div class="flex w-full h-full flex-col bg-gray-50 dark:bg-gray-900">
    <header-bar @toggle-mobile-menu="$emit('toggleMobileMenu')">
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
      <template #default>
        <n-button size="small" secondary @click="handleAddRule">
          <template #icon>
            <n-icon>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 5v14M5 12h14" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </n-icon>
          </template>
          {{ t('common.add') }}
        </n-button>
        <n-button size="small" secondary @click="handleImport" :title="t('common.import')">
          <template #icon>
            <n-icon>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" stroke-linejoin="round"/>
                <polyline points="7 10 12 15 17 10" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="12" y1="15" x2="12" y2="3" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </n-icon>
          </template>
        </n-button>
        <n-button size="small" secondary @click="handleExport" :title="t('rule.export')">
          <template #icon>
            <n-icon>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" stroke-linejoin="round"/>
                <polyline points="17 8 12 3 7 8" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="12" y1="3" x2="12" y2="15" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </n-icon>
          </template>
        </n-button>
      </template>
      <template #mobile-actions>
        <n-button size="small" @click="handleAddRule">
          <template #icon>
            <n-icon>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 5v14M5 12h14" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </n-icon>
          </template>
        </n-button>
        <n-button size="small" @click="handleImport">
          <template #icon>
            <n-icon>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" stroke-linejoin="round"/>
                <polyline points="7 10 12 15 17 10" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="12" y1="15" x2="12" y2="3" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </n-icon>
          </template>
        </n-button>
        <n-button size="small" @click="handleExport">
          <template #icon>
            <n-icon>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" stroke-linecap="round" stroke-linejoin="round"/>
                <polyline points="17 8 12 3 7 8" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="12" y1="3" x2="12" y2="15" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </n-icon>
          </template>
        </n-button>
      </template>
    </header-bar>

    <!-- Rules Container -->
    <div class="flex-1 overflow-y-auto p-4 md:p-6">
      <div v-if="rules.length === 0" class="flex flex-col items-center justify-center h-full text-gray-500 dark:text-gray-400">
        <n-icon size="64" class="mb-4 opacity-30">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M9 12h6m-6 4h6m2 5H7a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5.586a1 1 0 0 1 .707.293l5.414 5.414a1 1 0 0 1 .293.707V19a2 2 0 0 1-2 2z" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </n-icon>
        <p class="text-lg">{{ t('rule.noRules') }}</p>
        <n-button size="small" class="mt-4" @click="handleAddRule">
          {{ t('common.add') }}
        </n-button>
      </div>

      <VueDraggable
        v-else
        v-model="rules"
        class="space-y-3"
        handle=".drag-handle"
        animation="200"
        ghost-class="rule-card-ghost"
        @end="handleDragEnd"
      >
        <div
          v-for="rule in rules"
          :key="rule.uuid"
          class="rule-card group"
          :class="{ 'editing': editingRuleId === rule.uuid }"
        >
          <!-- View Mode -->
          <div v-if="editingRuleId !== rule.uuid" class="rule-card-content" @click="startEditing(rule.uuid)">
            <!-- Drag Handle -->
            <div class="drag-handle">
              <n-icon size="16">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="9" cy="5" r="1"/>
                  <circle cx="9" cy="12" r="1"/>
                  <circle cx="9" cy="19" r="1"/>
                  <circle cx="15" cy="5" r="1"/>
                  <circle cx="15" cy="12" r="1"/>
                  <circle cx="15" cy="19" r="1"/>
                </svg>
              </n-icon>
            </div>

            <!-- Action Badge -->
            <div class="rule-action-badge" :style="{ backgroundColor: getActionColor(rule.action) }">
              <n-icon size="14">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                  <path :d="getActionIcon(rule.action)"/>
                </svg>
              </n-icon>
              <span class="text-xs font-semibold">{{ rule.action.toUpperCase() }}</span>
            </div>

            <!-- Rule Type -->
            <div class="rule-type">
              <n-tag size="small" :bordered="false">
                {{ ruleTypeOptions.find(opt => opt.value === rule.ruleType)?.label || rule.ruleType }}
              </n-tag>
            </div>

            <!-- Pattern -->
            <div class="rule-pattern">
              <code class="pattern-code">{{ rule.pattern || '<empty>' }}</code>
            </div>

            <!-- Actions -->
            <div class="rule-actions">
              <n-button text size="small" @click.stop="startEditing(rule.uuid)">
                <template #icon>
                  <n-icon>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" stroke-linecap="round" stroke-linejoin="round"/>
                      <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                  </n-icon>
                </template>
              </n-button>
              <n-button text size="small" @click.stop="handleRemoveRule(rule.uuid)">
                <template #icon>
                  <n-icon>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                  </n-icon>
                </template>
              </n-button>
            </div>
          </div>

          <!-- Edit Mode -->
          <div v-else class="rule-card-edit">
            <div class="flex flex-col md:flex-row gap-3 flex-1 items-center w-full">
              <!-- Action Select -->
              <n-select
                v-model:value="rule.action"
                :options="actionOptions"
                size="small"
                class="flex-shrink-0 w-24"
              />

              <!-- Rule Type Select -->
              <n-select
                v-model:value="rule.ruleType"
                :options="ruleTypeOptions"
                size="small"
                class="flex-shrink-0 w-40"
              />

              <!-- Pattern Input -->
              <n-input
                v-model:value="rule.pattern"
                size="small"
                placeholder="Enter pattern..."
                class="flex-1 min-w-0"
                @keyup.enter="handleUpdateRule(rule)"
              />
            </div>

            <!-- Edit Actions -->
            <div class="flex gap-2 mt-3 md:mt-0 flex-shrink-0">
              <n-button size="small" secondary type="success" :loading="isLoading" @click="handleUpdateRule(rule)">
                <template #icon>
                  <n-icon>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M20 6L9 17l-5-5" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                  </n-icon>
                </template>
                {{ t('common.update') }}
              </n-button>
              <n-button size="small" @click="cancelEditing(rule.uuid)">
                {{ t('common.cancel') }}
              </n-button>
            </div>
          </div>
        </div>
      </VueDraggable>
    </div>
  </div>
</template>

<style scoped lang="scss">
.rule-card {
  @apply relative;
  @apply transition-all duration-200;
  @apply bg-white dark:bg-gray-800;
  @apply rounded-xl;
  @apply shadow-sm hover:shadow-md;
  @apply border border-transparent hover:border-gray-200 dark:hover:border-gray-600;

  &:hover {
    @apply -translate-y-0.5;
  }

  &.editing {
    @apply shadow-md border-primary/30;
    @apply bg-white/95 dark:bg-gray-800/95;
  }
}

.rule-card-ghost {
  @apply opacity-50 bg-primary/5;
  @apply border-dashed border-2 border-primary;
}

.rule-card-content {
  @apply flex items-center gap-3;
  @apply p-3 md:p-4;
  @apply cursor-pointer;
}

.drag-handle {
  @apply cursor-grab;
  @apply text-gray-400 dark:text-gray-500;
  @apply opacity-0 group-hover:opacity-100;
  @apply transition-opacity duration-150;

  &:active {
    @apply cursor-grabbing;
  }
}

.rule-action-badge {
  @apply flex items-center gap-1.5;
  @apply px-2.5 py-1;
  @apply rounded-lg;
  @apply text-white;
  @apply shadow-sm;
  min-width: 90px;
}

.rule-type {
  @apply flex-shrink-0;
}

.rule-pattern {
  @apply flex-1 min-w-0;
  @apply px-3 py-1.5;
  @apply rounded-lg;
  @apply bg-gray-100 dark:bg-gray-700;
}

.pattern-code {
  @apply text-sm;
  @apply text-gray-800 dark:text-gray-200;
  @apply font-mono;
  @apply block truncate;
}

.rule-actions {
  @apply flex items-center gap-1;
  @apply opacity-0 group-hover:opacity-100;
  @apply transition-opacity duration-150;

  @screen md {
    @apply opacity-100;
  }
}

.rule-card-edit {
  @apply flex flex-col md:flex-row items-start md:items-center gap-3;
  @apply p-4;
  @apply w-full;

  :deep(.n-select) {
    @apply flex-shrink-0;
  }

  :deep(.n-input) {
    @apply min-w-0;
  }
}

// Loading state
.rule-card.loading {
  @apply pointer-events-none opacity-60;
}
</style>
