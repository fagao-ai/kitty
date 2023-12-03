import { ref, watch } from 'vue'
import { darkTheme } from 'naive-ui'
import type { BuiltInGlobalTheme } from 'naive-ui/es/themes/interface'
import { NConfigProvider, GlobalThemeOverrides, useOsTheme } from 'naive-ui'

const useTheme = () => {
    const osThemeRef = useOsTheme()
    const theme = ref<BuiltInGlobalTheme | null>(null)
    const primaryColor = '#5352ed'
    const themeOverrides: GlobalThemeOverrides = {
        common: {
            primaryColor: primaryColor,
            primaryColorHover: primaryColor
        },
        Button: {
            textColorPrimary: primaryColor,
            textColor: primaryColor
        }
    }

    watch(() => osThemeRef.value, (value) => {
        if (value === 'dark') {
            document.documentElement.classList.add("dark")
            theme.value = darkTheme
            return
        }
        document.documentElement.classList.remove("dark")
        theme.value = null
    }, { immediate: true })

    return {
        theme,
        themeOverrides
    }
}

export { useTheme }
