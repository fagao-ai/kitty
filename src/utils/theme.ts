import { ref, watch } from 'vue'
import { type GlobalThemeOverrides, darkTheme, useOsTheme } from 'naive-ui'
import type { BuiltInGlobalTheme } from 'naive-ui/es/themes/interface'

function useTheme() {
  const osThemeRef = useOsTheme()
  const theme = ref<BuiltInGlobalTheme | null>(null)
  const primaryColor = '#5352ed'
  const lightThemeOverrides: GlobalThemeOverrides = {
    common: {
      primaryColor,
      primaryColorHover: primaryColor,
    },
    Button: {
      textColorPrimary: primaryColor,
      textColor: primaryColor,
    },
    Menu: {
      itemTextColorActive: 'whitesmoke',
      itemTextColorActiveHover: 'white',
    },
  }

  const darkThemeOverrides: GlobalThemeOverrides = {
    common: {
      primaryColor,
      primaryColorHover: primaryColor,
    },
    Button: {
      textColorPrimary: primaryColor,
      textColor: primaryColor,
    },
    Menu: {
      itemTextColor: '#5b7497',
      itemTextColorActive: 'whitesmoke',
      itemTextColorActiveHover: 'white',
    },
  }

  watch(() => osThemeRef.value, (value) => {
    if (value === 'dark') {
      document.documentElement.classList.add('dark')
      theme.value = darkTheme
      return
    }
    document.documentElement.classList.remove('dark')
    theme.value = null
  }, { immediate: true })

  return {
    theme,
    lightThemeOverrides,
    darkThemeOverrides,
  }
}

export { useTheme }
