<script setup lang="ts">
import { NButton, NForm, NFormItem, NInput } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import type { ProxyAdd } from '@/types/proxy'
import { invoke } from '@/utils/invoke'

const props = withDefaults(defineProps<ProxyAdd>(), { showModal: false })

const emits = defineEmits<Emits>()

interface Emits {
  (e: 'insertSubmit'): void
}

const showInsertModal = useVModel(props, 'showModal')

const form = useVModel(props, 'formData')

async function onInsertSubmit() {
  await invoke('add_hy_item', { hysteria_config: form.value })
  showInsertModal.value = false
  emits('insertSubmit')
}

function onCancelInsert() {
  showInsertModal.value = false
}
</script>

<template>
  <n-modal
    v-model:show="showInsertModal"
    class="w-1/2 h-1/2"
    :mask-closable="false"
    preset="card"
    title="添加代理"
    size="huge"
    :bordered="false"
    :segmented="true"
  >
    <n-form
      :model="form"
      size="medium"
      label-placement="left"
      label-width="auto"
    >
      <n-form-item
        label="服务地址"
        path="server"
      >
        <n-input v-model:value="form.server" />
      </n-form-item>
      <n-form-item
        label="认证"
        path="auth"
      >
        <n-input
          v-model:value="form.auth"
          placeholder="认证密码"
        />
      </n-form-item>
      <n-form-item
        label="上行"
        path="bandwidth.up"
      >
        <n-input v-model:value="form.bandwidth.up" />
      </n-form-item>
      <n-form-item
        label="下行"
        path="bandwidth.down"
      >
        <n-input v-model:value="form.bandwidth.down" />
      </n-form-item>
      <n-form-item
        label="sni"
        path="tls.sni"
      >
        <n-input v-model:value="form.tls.sni" />
      </n-form-item>
      <n-form-item
        label="安全连接"
        path="tls.insecure"
      >
        <n-switch v-model:value="form.tls.insecure" />
      </n-form-item>
    </n-form>
    <template #footer>
      <div class="w-full flex flex-center gap-16">
        <n-button
          round
          @click="onCancelInsert"
        >
          取消
        </n-button>
        <n-button
          round
          type="primary"
          @click="onInsertSubmit"
        >
          添加
        </n-button>
      </div>
    </template>
  </n-modal>
</template>
