import { useMessageStore } from '~/store'
import { FetchError } from 'ofetch'

export default defineNuxtPlugin((nuxtApp) => {
  const messageStore = useMessageStore()
  nuxtApp.vueApp.config.errorHandler = (error) => {
    if (typeof error === 'object' && error !== null && isNuxtError(error)) {
      messageStore.message = {
        type: 'error',
        message: error.message,
      }
    }
    if (error instanceof FetchError) {
      messageStore.message = {
        type: 'error',
        message: String(error.data),
      }
    }
  }
})
