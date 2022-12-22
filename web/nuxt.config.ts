// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  css: ['assets/styles/main.sass'],
  runtimeConfig: {
    public: {
      API_HTTP_BASE_URL: process.env.API_HTTP_BASE_URL,
    },
  },
})
