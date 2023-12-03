<template>
	<div class="flex flex-col w-full h-full">
		<div class="h-16 flex justify-between items-center">
			<div class="text-primay text-2xl">proxies</div>
			<div>
				<n-button
					round
					@click="onClickInsertButton"
				>add</n-button>
			</div>
		</div>
		<div class="flex-1 w-full">
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
	</div>
	<add-proxy-modal
		:form-data="formValue"
		v-model:showModal="showInsertModal"
	/>
</template>

<script setup lang="ts">
import { NButton } from 'naive-ui'
import { reactive, ref } from 'vue'
import ProxyCard from './ProxyCard.vue'
import type { ProxyCard as Card, HysteriaProxy } from '@/types/proxy'
import AddProxyModal from '@/views/proxy/AddProxyModal.vue'

const showInsertModal = ref(false)



const onClickInsertButton = () => {
	showInsertModal.value = true
}

const formValue = reactive<HysteriaProxy>({
	serverAddress: "ip:port",
	auth: "password",
	bandWidth: {
		up: "10 mbps",
		down: "100 mbps"
	},
	tls: {
		sni: "bing.com",
		insecure: true
	},
	socks5: {
		listen: "127.0.0.1:1080"
	},
	http: {
		listen: "127.0.0.1:8080"
	}
})

const cards: Card[] = [
	{
		tag: 'Vmess',
		name: '美国硅谷',
		delay: 144,
		protocol: 'TCP'
	},
	{
		tag: 'tag2',
		name: 'name2',
		delay: 143,
		protocol: 'UDP'
	},
	{
		tag: 'tag3',
		name: 'name3',
		delay: 142,
		protocol: 'TCP'
	},
	{
		tag: 'tag4',
		name: 'name4',
		delay: 141,
		protocol: 'UDP'
	},
	{
		tag: 'tag5',
		name: 'name5',
		delay: 140,
		protocol: 'UDP'
	},
	{
		tag: 'tag6',
		name: 'name6',
		delay: 139,
		protocol: 'TCP'
	},
	{
		tag: 'tag7',
		name: 'name7',
		delay: 138,
		protocol: 'UDP'
	},
	{
		tag: 'tag8',
		name: 'name8',
		delay: 500,
		protocol: 'UDP'
	},
	{
		tag: 'tag9',
		name: 'name9',
		delay: 666,
		protocol: 'TCP'
	},
	{
		tag: 'tag10',
		name: 'name10',
		delay: 1428,
		protocol: 'UDP'
	},
]
</script>