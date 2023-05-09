module.exports = {
  root: true,

  env: {
    browser: true,
    node: true,
  },

  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
    parser: '@typescript-eslint/parser',
  },

  extends: [
    'plugin:@typescript-eslint/recommended',
    'plugin:nuxt/recommended',
    'plugin:vue/vue3-recommended',
    'plugin:prettier/recommended',
  ],

  rules: {
    'vue/script-setup-no-uses-vars': 'off',
    'vue/no-v-html': 'off',
  },

  overrides: [
    {
      files: ['layouts/*.vue', 'pages/**/*.vue'],
      rules: {
        'vue/multi-word-component-names': 'off',
      },
    },
    {
      files: ['layouts/*.vue', 'pages/**/*.vue', 'components/*.vue'],
      rules: {
        'vue/valid-v-slot': 'off',
        'vue/no-template-shadow': 'off',
      },
    },
    {
      files: [
        'layouts/*.vue',
        'pages/**/*.vue',
        'components/*.vue',
        'composables/*.ts',
        'store/*.ts',
      ],
      rules: {
        '@typescript-eslint/no-explicit-any': 'off',
        '@typescript-eslint/no-non-null-assertion': 'off',
        '@typescript-eslint/no-unused-vars': [
          'warn',
          { argsIgnorePattern: '^_' },
        ],
      },
    },
  ],
}
