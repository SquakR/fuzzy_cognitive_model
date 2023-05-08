import { useMessageStore } from '~/store'

export default defineNuxtPlugin((nuxtApp) => {
  const messageStore = useMessageStore()
  nuxtApp.vueApp.config.errorHandler = (error) => {
    if (typeof error === 'object' && error !== null && isNuxtError(error)) {
      messageStore.message = {
        type: 'error',
        message: error.message,
      }
    }
  }
})
