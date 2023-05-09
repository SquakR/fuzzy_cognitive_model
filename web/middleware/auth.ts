import { useUserStore } from '~/store'

export default defineNuxtRouteMiddleware(async () => {
  const userStore = useUserStore()
  if (!userStore.user) {
    return navigateTo({ name: 'auth-sign_in' })
  }
})
