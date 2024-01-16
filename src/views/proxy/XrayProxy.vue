<script setup lang="ts">
import ProxyCard from '@/views/proxy/ProxyCard.vue'
import Empty from '@/components/Empty.vue'
import type { ProxyCard as Card } from '@/types/proxy'

interface Props {
  data: Card[]
}

defineProps<Props>()
</script>

<template>
  <div class="w-full h-full">
    <template v-if="data.length !== 0">
      <div class="grid grid-cols-5 auto-rows-fr gap-4 xl:grid-cols-6 xxl:grid-cols-7 xxxl:grid-cols-8 tv:grid-cols-10">
        <template
          v-for="card, index in data"
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
    </template>
    <empty
      v-else
      description="Xray No Proxy Found"
    />
  </div>
</template>
