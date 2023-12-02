<template>
	<div class="flex flex-col w-full h-full">
		<div class="h-16 flex justify-between items-center">
			<div class="text-primay text-2xl">代理</div>
			<div>
				<n-button @click="onClickInsertButton">添加</n-button>
			</div>
		</div>
		<div
			class="flex-1 w-full h-full grid grid-cols-5 gap-4 xl:grid-cols-6 xxl:grid-cols-7 xxxl:grid-cols-8 tv:grid-cols-10"
		>
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
			:model="formValue"
			size="medium"
		><n-form-item
				label="姓名"
				path="user.name"
			>
				<n-input
					v-model:value="formValue.user.name"
					placeholder="输入姓名"
				/>
			</n-form-item>
			<n-form-item
				label="年龄"
				path="user.age"
			>
				<n-input
					v-model:value="formValue.user.age"
					placeholder="输入年龄"
				/>
			</n-form-item>
			<n-form-item
				label="电话号码"
				path="phone"
			>
				<n-input
					v-model:value="formValue.phone"
					placeholder="电话号码"
				/>
			</n-form-item>
			<n-form-item>
				<n-button
					attr-type="button"
					@click="() => { }"
				>
					验证
				</n-button>
			</n-form-item>
		</n-form>
		<template #footer>
			<n-button @click="onCancelInsert">
				取消
			</n-button>
			<n-button
				type="primary"
				@click="onInsertSubmit"
			>
				提交
			</n-button>
		</template>
	</n-modal>
</template>

<script setup lang="ts">
import { NForm, NFormItem, NInput, NButton } from 'naive-ui'
import { reactive, ref } from 'vue'
import ProxyCard from './ProxyCard.vue'
import type { ProxyCard as Card } from '@/types/proxy'

const showInsertModal = ref(false)

const onInsertSubmit = () => {
	console.log('onInsertSubmit')
}

const onCancelInsert = () => {
	showInsertModal.value = false
}

const onClickInsertButton = () => {
	showInsertModal.value = true
}

const formValue = reactive({
	user: {
		name: '召召',
		age: '18'
	},
	phone: '110'
})

const cards: Card[] = [
	{
		tag: 'tag1',
		name: 'name1',
		delay: 'delay1',
		protocol: 'protocol1'
	},
	{
		tag: 'tag2',
		name: 'name2',
		delay: 'delay2',
		protocol: 'protocol2'
	},
	{
		tag: 'tag3',
		name: 'name3',
		delay: 'delay3',
		protocol: 'protocol3'
	},
	{
		tag: 'tag4',
		name: 'name4',
		delay: 'delay4',
		protocol: 'protocol4'
	},
	{
		tag: 'tag5',
		name: 'name5',
		delay: 'delay5',
		protocol: 'protocol5'
	},
	{
		tag: 'tag6',
		name: 'name6',
		delay: 'delay6',
		protocol: 'protocol6'
	},
	{
		tag: 'tag7',
		name: 'name7',
		delay: 'delay7',
		protocol: 'protocol7'
	},
	{
		tag: 'tag8',
		name: 'name8',
		delay: 'delay8',
		protocol: 'protocol8'
	},
	{
		tag: 'tag9',
		name: 'name9',
		delay: 'delay9',
		protocol: 'protocol9'
	},
	{
		tag: 'tag10',
		name: 'name10',
		delay: 'delay10',
		protocol: 'protocol10'
	},
]
</script>