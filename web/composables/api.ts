import { User } from '~~/types'

export type UseAPIUserVariables = {
  userId: number
}

export const useAPIUser = () => {
  const data = ref<User | null>(null)
  return {
    data,
    async execute({ userId }: UseAPIUserVariables) {
      try {
        data.value = await localFetch(`/user/${userId}`)
      } catch (e) {
        data.value = null
        throw e
      }
    },
  }
}
