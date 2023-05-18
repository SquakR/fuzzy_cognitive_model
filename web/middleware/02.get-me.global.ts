import { useUserStore } from '~/store'

export default defineNuxtRouteMiddleware(async () => {
  const userStore = useUserStore()
  await userStore.updateMe()
})
