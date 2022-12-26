import { User } from '~~/types'
import { FetchError } from 'ofetch'
import { useGlobalMessagesStore } from '~~/store/global-messages'

export type UseAPIUserVariables = {
  userId: number
}

export const useAPIUser = () => {
  const pending = ref(false)
  const data = ref<User | null>(null)
  const globalMessageStore = useGlobalMessagesStore()
  return {
    pending,
    data,
    async execute({ userId }: UseAPIUserVariables) {
      pending.value = true
      try {
        data.value = await localFetch(`/user/${userId}`)
      } catch (e) {
        if (e instanceof FetchError) {
          globalMessageStore.addFetchErrorMessage(e, Infinity)
        } else {
          throw e
        }
      }
      pending.value = false
    },
  }
}
