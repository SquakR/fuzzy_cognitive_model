// https://nuxt.com/docs/api/configuration/nuxt-config
import VueI18nPlugin from '@intlify/unplugin-vue-i18n'

export default defineNuxtConfig({
  css: [
    'vuetify/lib/styles/main.sass',
    '@mdi/font/css/materialdesignicons.min.css',
  ],
  build: {
    transpile: ['vuetify'],
  },
  vite: {
    plugins: [VueI18nPlugin.vite({})],
  },
  runtimeConfig: {
    public: {
      API_HTTP_BASE_URL: process.env.API_HTTP_BASE_URL,
    },
  },
  modules: ['@vueuse/nuxt', '@pinia/nuxt'],
})
