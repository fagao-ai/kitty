import antfu from '@antfu/eslint-config'

export default antfu(
  {
    // Enable stylistic formatting rules
    // stylistic: true,

    // Or customize the stylistic rules
    stylistic: {
      indent: 2, // 4, or 'tab'
      quotes: 'single', // or 'double'
    },

    // TypeScript and Vue are auto-detected, you can also explicitly enable them:
    typescript: true,
    vue: true,

    // Disable jsonc and yaml support
    jsonc: false,
    yaml: false,

    // `.eslintignore` is no longer supported in Flat config, use `ignores` instead
    ignores: [
      './fixtures',
      './node_modules/',
      './dist/',
      '.vscode/',
      '.idea/',
      './src-tauri/',
      '.cargo',
      './public/',
    ],
  },
  {
    files: ['src/*.vue', 'src/**/*.vue', 'src/**/**/*.vue'],
    rules: {
      'vue/component-name-in-template-casing': ['error', 'kebab-case', {
        registeredComponentsOnly: true,
        ignores: [],
      }],
    },
  },
  {
    rules: {
      'no-tabs': ['error', { allowIndentationTabs: true }],
    },
  },
)
