<script setup lang="ts">
import { ref } from 'vue'
import { NSwitch } from 'naive-ui'
import { invoke } from '@/utils/invoke'

const proxyStatus = ref(false)
const proxyLoading = ref(false)
async function handleSwitchProxy(value: boolean) {
  proxyLoading.value = true
  try {
    if (value)
      await invoke('start_hysteria')
    else
      await invoke('stop_hy')
  }
  finally {
    proxyLoading.value = false
  }
}

const onlyAllowNumber = (value: string) => !value || /^\d+$/.test(value)
</script>

<template>
  <div class="w-full h-full flex flex-col space-y-4">
    <div class="h-8 text-2xl text-primay font-extrabold">
      Settings
    </div>
    <div class="flex-1 flex flex-col space-y-6">
      <div class="grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 bg-[#f9f7f7] shadow-lg rounded-md text-[#5b7497]">
        <div class="flex justify-between">
          <div class="font-semibold">
            开机启动
          </div>
          <div class="font-medium">
            <n-switch :value="false" :disabled="true" size="medium" />
          </div>
        </div>
        <div class="flex justify-between">
          <div class="font-semibold">
            Language
          </div>
          <div class="font-medium">
            Chinese
          </div>
        </div>
        <div class="flex justify-between">
          <div class="font-semibold">
            系统代理
          </div>
          <div class="font-medium">
            <n-switch
              v-model="proxyStatus"
              :loading="proxyLoading"
              size="medium"
              @update:value="handleSwitchProxy"
            />
          </div>
        </div>
        <div class="flex justify-between">
          <div class="font-semibold">
            允许局域网连接
          </div>
          <div class="font-medium">
            Off
          </div>
        </div>
      </div>
      <div class="grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 text-[#5b7497] bg-[#f9f7f7] shadow-lg rounded-md">
        <div class="flex justify-between">
          <div class="font-semibold">
            代理模式
          </div>
          <div class="font-medium">
            全局
          </div>
        </div>
        <div class="flex justify-between">
          <div class="font-semibold">
            Socks5代理端口
          </div>
          <div class="font-medium w-20">
            <n-input
              type="text"
              value="10086"
              :disabled="true"
              :allow-input="onlyAllowNumber"
            />
          </div>
        </div>
        <div class="flex justify-between">
          <div class="font-semibold">
            HTTP代理端口
          </div>
          <div class="font-medium w-20">
            <n-input
              type="text"
              value="10086"
              :disabled="true"
              :allow-input="onlyAllowNumber"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
