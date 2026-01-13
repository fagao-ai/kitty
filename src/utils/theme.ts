import { ref, watch } from 'vue'

function useTheme() {
  const isDark = ref(false)

  // 检测系统主题
  const detectTheme = () => {
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
      isDark.value = true
    }
    else {
      isDark.value = false
    }
  }

  // 监听系统主题变化
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  mediaQuery.addEventListener('change', detectTheme)

  // 初始检测
  detectTheme()

  // 监听 isDark 变化,更新 DOM
  watch(isDark, (value) => {
    if (value) {
      document.documentElement.classList.add('dark')
    }
    else {
      document.documentElement.classList.remove('dark')
    }
  }, { immediate: true })

  return {
    isDark,
  }
}

export { useTheme }
