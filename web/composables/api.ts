import { User } from '~~/types'

export type UseAPIUserVariables = {
  userId: number
}

export function useAPIUser() {
  const pending = ref(false)
  const data = ref<User | null>(null)
  return {
    pending,
    data,
    async execute({ userId }: UseAPIUserVariables) {
      pending.value = true
      try {
        data.value = await localFetch(`/user/${userId}`)
      } catch {
        data.value = null
      }
      pending.value = false
    },
  }
}
