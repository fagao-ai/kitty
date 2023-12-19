<script setup lang="ts">
import { NButton } from 'naive-ui'
import { reactive, ref } from 'vue'
import ProxyCard from '@/views/proxy/ProxyCard.vue'
import type { ProxyCard as Card, HysteriaProxy } from '@/types/proxy'
import AddProxyModal from '@/views/proxy/AddProxyModal.vue'
import { invoke } from '@/utils/invoke'

const showInsertModal = ref(false)

const formValue = reactive<HysteriaProxy>({
  name: '名称',
  server: 'ip:port',
  auth: 'password',
  bandwidth: {
    up: '10 mbps',
    down: '100 mbps',
  },
  tls: {
    sni: 'bing.com',
    insecure: true,
  },
})

const cards = ref<Card[]>([])

async function batchGetProxy() {
  const res = await invoke<Card[]>('get_all_proxies')
  cards.value = res.data.map(item => ({
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

batchGetProxy()
</script>

<template>
  <div class="flex flex-col w-full h-full space-y-4">
    <div class="h-8 flex justify-between items-center">
      <div class="text-primay text-2xl font-extrabold">
        Proxies
      </div>
      <div>
        <n-button
          round
          @click="showInsertModal = true"
        >
          add
        </n-button>
      </div>
    </div>
    <div
      v-if="cards.length !== 0"
      class="flex-1 w-full"
    >
      <div class="grid grid-cols-5 auto-rows-fr gap-4 xl:grid-cols-6 xxl:grid-cols-7 xxxl:grid-cols-8 tv:grid-cols-10">
        <template
          v-for="card, index in cards"
          :key="index"
        >
          <proxy-card
            :name="card.name"
            :delay="card.delay"
            :tag="card.tag"
            :protocol="card.protocol"
          />
        </template>
      </div>
    </div>
    <div
      v-else
      class="flex-1 w-full flex justify-center items-center"
    >
      <n-empty
        size="huge"
        description="No Proxy Found"
      />
    </div>
  </div>
  <add-proxy-modal
    v-model:showModal="showInsertModal"
    :form-data="formValue"
    @insert-submit="batchGetProxy"
  />
</template>
