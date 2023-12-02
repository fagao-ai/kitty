<template>
	<div class="flex flex-col h-full p-4">
		<div class="flex-1 flex flex-col">
			<div class="text-5xl font-bold h-1/6 flex flex-center text-primay">Kitty</div>
			<div class="flex-1 text-white text-lg">
				<n-menu
					default-value="proxy"
					:options="menuOptions"
					class="rounded-full"
				/>
			</div>
		</div>
		<div class="h-1/8 flex flex-center flex-col">
			<div class="text-primay text-lg">Kitty Version</div>
			<div class="text-primay">0.0.1.beta</div>
		</div>
	</div>
</template>
<script setup lang="ts">
import { h } from 'vue'
import { NMenu } from 'naive-ui'
import type { MenuOption } from 'naive-ui'
import { RouterLink } from 'vue-router'

const menuOptions: MenuOption[] = [
	{
		label: () =>
			h(
				RouterLink,
				{
					to: {
						name: 'proxy',
					},
				},
				{ default: () => 'proxies' },
			),
		key: 'proxy',
	},
]
</script>

<style scoped lang="scss">
:deep(.n-menu) {
	--n-item-color-active: #5352ed !important;
	--n-item-color-active-hover: #5352ed !important;

	.n-menu-item-content--selected::before {
		@apply rounded-full;
		// background-color: aqua
	}

	.n-menu-item-content {
		@apply flex justify-center items-center;
		padding-left: 0 !important;
		padding-right: 0 !important;
	}

	.n-menu-item-content-header {
		& a {
			font-size: 18px;
			color: white !important;
		}
	}
}
</style>