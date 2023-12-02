import { ref, watch } from 'vue'
import { darkTheme } from 'naive-ui'
import type { BuiltInGlobalTheme } from 'naive-ui/es/themes/interface'
import { NConfigProvider, GlobalThemeOverrides, useOsTheme } from 'naive-ui'

const useTheme = () => {
    const osThemeRef = useOsTheme()
    const theme = ref<BuiltInGlobalTheme | null>(null)

    const themeOverrides: GlobalThemeOverrides = {
        common: {
            primaryColor: '#5352ed'
        },
        Button: {
            textColor: '#5352ed'
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
