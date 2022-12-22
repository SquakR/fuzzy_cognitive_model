import { Ref } from 'vue'
import { User } from '~~/types'

export function useAPIUser(userId: Ref<number>) {
  const pending = ref(false)
  const data = ref<User | null>(null)
  return {
    pending,
    data,
    async execute() {
      pending.value = true
      try {
        data.value = await localFetch(`/user/${userId.value}`)
      } catch {
        data.value = null
      }
      pending.value = false
    },
  }
}
