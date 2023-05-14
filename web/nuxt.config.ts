// https://nuxt.com/docs/api/configuration/nuxt-config
import VueI18nPlugin from '@intlify/unplugin-vue-i18n'

export default defineNuxtConfig({
  css: [
    'vuetify/lib/styles/main.sass',
    '@mdi/font/css/materialdesignicons.min.css',
  ],
  build: {
    transpile: ['vuetify', 'rxjs'],
  },
  vite: {
    plugins: [VueI18nPlugin.vite({})],
  },
  runtimeConfig: {
    public: {
      API_HTTP_BASE_URL: process.env.API_HTTP_BASE_URL,
      API_WS_BASE_URL: process.env.API_WS_BASE_URL,
    },
  },
  typescript: {
    tsConfig: {
      compilerOptions: {
        types: ['vuetify'],
      },
    },
  },
  modules: ['@vueuse/nuxt', '@pinia/nuxt', '@vee-validate/nuxt'],
})
