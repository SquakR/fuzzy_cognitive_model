import { User } from '~~/types'
import { FetchError } from 'ofetch'

export type UseAPIUserVariables = {
  userId: number
}

export const useAPIUser = () => {
  const pending = ref(false)
  const data = ref<User | null>(null)
  const fetchResult = useFetchResult({ isGlobal: true })
  return {
    pending,
    data,
    async execute({ userId }: UseAPIUserVariables) {
      pending.value = true
      try {
        data.value = await localFetch(`/user/${userId}`)
      } catch (e) {
        if (e instanceof FetchError) {
          fetchResult.updateResult({ success: null, fetchError: e })
        } else {
          throw e
        }
      }
      pending.value = false
    },
  }
}
