import { useMessageStore } from '~/store'

export default defineNuxtRouteMiddleware(async () => {
  useMessageStore()
})
