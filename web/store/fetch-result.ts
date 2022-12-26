import { reactive } from 'vue'
import { defineStore } from 'pinia'
import { UseFetchResultReturn } from '~~/composables/fetch-result'

export const useFetchResultStore = defineStore('fetch-result', () => {
  const fetchResults = ref<UseFetchResultReturn[]>([])

  const addFetchResult = (fetchResult: UseFetchResultReturn) => {
    fetchResults.value.push(reactive(fetchResult))
    const index = fetchResults.value.length - 1
    return () => {
      fetchResults.value = fetchResults.value.filter((_, i) => i !== index)
    }
  }

  const activeFetchResults = computed(() => [
    ...fetchResults.value.filter((fetchResult) =>
      Boolean(fetchResult.result.fetchError)
    ),
    ...fetchResults.value.filter((fetchResult) =>
      Boolean(fetchResult.result.success)
    ),
  ])

  return { fetchResults, addFetchResult, activeFetchResults }
})
