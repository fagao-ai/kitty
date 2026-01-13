import { reactive, ref, toRaw } from 'vue'
import { decamelizeKeys } from 'humps'
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'
import { invoke } from '@/utils/invoke'
import type { KittyBaseConfig } from '@/types/setting'
import { setProxy } from '@/apis/proxy'

export function useConfig() {
  const loading = ref(true)
  const proxyLoading = ref(false)
  const baseConfig = reactive<KittyBaseConfig>({
    id: 0,
    localIp: '127.0.0.1',
    httpPort: 10086,
    socksPort: 10087,
    delayTestUrl: 'https://gstatic.com/generate_204',
    sysproxyFlag: false,
    autoStart: false,
    language: 'zh-CN',
    allowLan: false,
    mode: 'Rules',
    updateInterval: 3,
  })

  async function getBaseConfig() {
    const config = await invoke<KittyBaseConfig>('query_base_config')
    return config
  }

  async function initConfig() {
    const config = await getBaseConfig()

    Object.assign(baseConfig, config.data)

    loading.value = false
  }

  async function handleSwitchProxy(value: boolean) {
    proxyLoading.value = true
    try {
      await setProxy(value)
    }

    catch (_) {
      baseConfig.sysproxyFlag = false
    }
    finally {
      proxyLoading.value = false
    }
  }

  async function handleSwitchAutoStart(value: boolean) {
    if (value)
      await enable()

    else
      await disable()
  }

  async function handleBaseConfigUpdate() {
    await invoke('update_base_config', { record: decamelizeKeys(toRaw(baseConfig)) })
  }

  return {
    loading,
    proxyLoading,
    baseConfig,
    handleSwitchProxy,
    handleSwitchAutoStart,
    handleBaseConfigUpdate,
    initConfig,
    isEnabled,
  }
}
