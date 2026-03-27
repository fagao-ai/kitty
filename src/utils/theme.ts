import { computed, watch } from 'vue'
import { usePreferredDark } from '@vueuse/core'

function useTheme() {
  const isDark = usePreferredDark()

  watch(isDark, (value) => {
    document.documentElement.classList.toggle('dark', value)
  }, { immediate: true })

  return {
    isDark: computed(() => isDark.value),
  }
}

export { useTheme }
