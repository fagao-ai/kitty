import { ref, watch } from 'vue'
import { type GlobalThemeOverrides, darkTheme, lightTheme, useOsTheme } from 'naive-ui'
import type { BuiltInGlobalTheme } from 'naive-ui/es/themes/interface'

function useTheme() {
  const osThemeRef = useOsTheme()
  const theme = ref<BuiltInGlobalTheme | null>(null)

  // Design tokens
  const primaryColor = '#5352ed'
  const primaryColorHover = '#4544d4'
  const primaryColorActive = '#3b3ab8'
  const borderRadius = '8px'
  const borderRadiusSmall = '6px'

  const lightThemeOverrides: GlobalThemeOverrides = {
    common: {
      primaryColor,
      primaryColorHover,
      primaryColorPressed: primaryColorActive,
      primaryColorSuppl: primaryColor,
      infoColor: '#3b82f6',
      successColor: '#10b981',
      warningColor: '#f59e0b',
      errorColor: '#ef4444',
      textColor1: '#0f172a',
      textColor2: '#475569',
      textColor3: '#64748b',
      textColorDisabled: '#94a3b8',
      borderColor: '#e2e8f0',
      dividerColor: '#e2e8f0',
      borderRadius,
      borderRadiusSmall,
      boxShadow1: '0 2px 8px rgba(0, 0, 0, 0.08)',
      boxShadow2: '0 8px 16px rgba(0, 0, 0, 0.12)',
    },
    Button: {
      borderRadius,
      borderRadiusSmall,
      textColorPrimary: '#ffffff',
      textColorHoverPrimary: '#ffffff',
      textColorFocusPrimary: '#ffffff',
      textColorPressedPrimary: '#ffffff',
      textColor: primaryColor,
      border: '1px solid #e2e8f0',
      borderHover: '1px solid primaryColor',
      borderPressed: '1px solid primaryColorActive',
    },
    Card: {
      borderRadius,
      color: '#ffffff',
      colorModal: '#ffffff',
      colorTarget: '#f8fafc',
    },
    Menu: {
      itemColorActive: primaryColor,
      itemColorActiveHover: primaryColorHover,
      itemColorHover: '#f1f5f9',
      itemTextColor: '#475569',
      itemTextColorActive: '#ffffff',
      itemTextColorActiveHover: '#ffffff',
      itemTextColorChildTextColor: '#64748b',
      borderRadius,
      arrowColor: '#94a3b8',
      arrowColorHover: primaryColor,
      arrowColorActive: '#ffffff',
      arrowColorActiveHover: '#ffffff',
    },
    Switch: {
      railColor: '#e2e8f0',
      railColorActive: primaryColor,
    },
    Input: {
      borderRadius,
      border: '1px solid #e2e8f0',
      borderHover: '1px solid primaryColor',
      borderFocus: '1px solid primaryColor',
      color: '#ffffff',
      placeholderColor: '#94a3b8',
    },
    Select: {
      borderRadius,
      peers: {
        InternalSelection: {
          borderRadius,
          border: '1px solid #e2e8f0',
          borderHover: '1px solid primaryColor',
          borderActive: '1px solid primaryColor',
          color: '#ffffff',
          placeholderColor: '#94a3b8',
        },
      },
    },
    Modal: {
      borderRadius,
      boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
    },
    Tag: {
      borderRadiusSmall,
      color: '#f1f5f9',
      textColor: '#475569',
      textColorCheck: '#ffffff',
    },
  }

  const darkThemeOverrides: GlobalThemeOverrides = {
    common: {
      primaryColor,
      primaryColorHover,
      primaryColorPressed: primaryColorActive,
      primaryColorSuppl: primaryColor,
      infoColor: '#3b82f6',
      successColor: '#10b981',
      warningColor: '#f59e0b',
      errorColor: '#ef4444',
      textColor1: '#f1f5f9',
      textColor2: '#cbd5e1',
      textColor3: '#94a3b8',
      textColorDisabled: '#64748b',
      borderColor: '#334155',
      dividerColor: '#334155',
      borderRadius,
      borderRadiusSmall,
      boxShadow1: '0 2px 8px rgba(0, 0, 0, 0.3)',
      boxShadow2: '0 8px 16px rgba(0, 0, 0, 0.4)',
    },
    Button: {
      borderRadius,
      borderRadiusSmall,
      textColorPrimary: '#ffffff',
      textColorHoverPrimary: '#ffffff',
      textColorFocusPrimary: '#ffffff',
      textColorPressedPrimary: '#ffffff',
      textColor: primaryColor,
      border: '1px solid #334155',
      borderHover: '1px solid primaryColor',
      borderPressed: '1px solid primaryColorActive',
    },
    Card: {
      borderRadius,
      color: '#1e293b',
      colorModal: '#1e293b',
      colorTarget: '#0f172a',
    },
    Menu: {
      itemColorActive: primaryColor,
      itemColorActiveHover: primaryColorHover,
      itemColorHover: '#334155',
      itemTextColor: '#cbd5e1',
      itemTextColorActive: '#ffffff',
      itemTextColorActiveHover: '#ffffff',
      itemTextColorChildTextColor: '#94a3b8',
      borderRadius,
      arrowColor: '#64748b',
      arrowColorHover: primaryColor,
      arrowColorActive: '#ffffff',
      arrowColorActiveHover: '#ffffff',
    },
    Switch: {
      railColor: '#334155',
      railColorActive: primaryColor,
    },
    Input: {
      borderRadius,
      border: '1px solid #334155',
      borderHover: '1px solid primaryColor',
      borderFocus: '1px solid primaryColor',
      color: '#1e293b',
      placeholderColor: '#64748b',
    },
    Select: {
      borderRadius,
      peers: {
        InternalSelection: {
          borderRadius,
          border: '1px solid #334155',
          borderHover: '1px solid primaryColor',
          borderActive: '1px solid primaryColor',
          color: '#1e293b',
          placeholderColor: '#64748b',
        },
      },
    },
    Modal: {
      borderRadius,
      boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.5)',
    },
    Tag: {
      borderRadiusSmall,
      color: '#334155',
      textColor: '#cbd5e1',
      textColorCheck: '#ffffff',
    },
  }

  watch(osThemeRef, (value) => {
    if (value === 'dark') {
      document.documentElement.classList.add('dark')
      theme.value = darkTheme
      return
    }
    document.documentElement.classList.remove('dark')
    theme.value = lightTheme
  }, { immediate: true })

  return {
    theme,
    lightThemeOverrides,
    darkThemeOverrides,
  }
}

export { useTheme }
