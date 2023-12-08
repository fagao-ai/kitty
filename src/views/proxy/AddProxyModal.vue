<script setup lang="ts">
import { NButton, NForm, NFormItem, NInput } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import type { ProxyAdd } from '@/types/proxy'

const props = withDefaults(defineProps<ProxyAdd>(), { showModal: false })

const showInsertModal = useVModel(props, 'showModal')

const form = useVModel(props, 'formData')

function onInsertSubmit() {
  // eslint-disable-next-line no-console
  console.log('onInsertSubmit')
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
    <NForm
      :model="form"
      size="medium"
      label-placement="left"
      label-width="auto"
    >
      <NFormItem
        label="服务地址"
        path="serverAddress"
      >
        <NInput v-model:value="form.serverAddress" />
      </NFormItem>
      <NFormItem
        label="认证"
        path="auth"
      >
        <NInput
          v-model:value="form.auth"
          placeholder="认证密码"
        />
      </NFormItem>
      <NFormItem
        label="上行"
        path="bandWidth.up"
      >
        <NInput v-model:value="form.bandWidth.up" />
      </NFormItem>
      <NFormItem
        label="下行"
        path="bandWidth.down"
      >
        <NInput v-model:value="form.bandWidth.down" />
      </NFormItem>
      <NFormItem
        label="sni"
        path="tls.sni"
      >
        <NInput v-model:value="form.tls.sni" />
      </NFormItem>
      <NFormItem
        label="安全连接"
        path="tls.insecure"
      >
        <n-switch v-model:value="form.tls.insecure" />
      </NFormItem>
      <NFormItem
        label="socks5"
        path="socks5.listen"
      >
        <NInput v-model:value="form.socks5.listen" />
      </NFormItem>
      <NFormItem
        label="http"
        path="http.listen"
      >
        <NInput v-model:value="form.http.listen" />
      </NFormItem>
    </NForm>
    <template #footer>
      <div class="w-full flex flex-center gap-16">
        <NButton
          round
          @click="onCancelInsert"
        >
          取消
        </NButton>
        <NButton
          round
          type="primary"
          @click="onInsertSubmit"
        >
          提交
        </NButton>
      </div>
    </template>
  </n-modal>
</template>
