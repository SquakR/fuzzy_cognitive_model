import { ComputedRef, WritableComputedRef, Ref, UnwrapRef } from 'vue'
import { FetchError } from 'ofetch'
import { useFetchResultStore } from '~~/store'

export interface FetchResult {
  success: string | null
  fetchError: FetchError | null
}
export interface UseFetchResultOptions {
  isGlobal: boolean
  successInterval?: number
  fetchErrorInterval?: number
}
export interface UseFetchResultReturn {
  result: ComputedRef<FetchResult>
  updateResult: (newResult: FetchResult) => void
  clearResult: () => void
  successInterval: Ref<number>
  fetchErrorInterval: Ref<number>
}

export const useFetchResult = (
  options: UseFetchResultOptions
): UseFetchResultReturn => {
  const { interval: successInterval, value: success } = useTimeoutValue<string>(
    options.successInterval ?? 0
  )
  const { interval: fetchErrorInterval, value: fetchError } =
    useTimeoutValue<FetchError>(options.fetchErrorInterval ?? Infinity)

  const fetchResult = computed<FetchResult>(() => ({
    success: success.value,
    fetchError: fetchError.value,
  }))
  const updateFetchResult = (newResult: FetchResult) => {
    success.value = newResult.success
    fetchError.value = newResult.fetchError
  }
  const clearFetchResult = () => {
    success.value = null
    fetchError.value = null
  }

  const result: UseFetchResultReturn = {
    result: fetchResult,
    updateResult: updateFetchResult,
    clearResult: clearFetchResult,
    successInterval,
    fetchErrorInterval,
  }

  if (options.isGlobal) {
    const fetchResultStore = useFetchResultStore()
    let deleteFetchResult: ReturnType<typeof fetchResultStore.addFetchResult>
    onMounted(() => {
      deleteFetchResult = fetchResultStore.addFetchResult(result)
    })
    onUnmounted(() => {
      deleteFetchResult()
    })
  }

  return result
}

interface UseTimeoutValueReturn<T> {
  interval: Ref<number>
  value: WritableComputedRef<UnwrapRef<T> | null>
}
const useTimeoutValue = <T>(interval: number): UseTimeoutValueReturn<T> => {
  const intervalRef = ref(interval)
  const valueRef = ref<T | null>(null)

  const { isPending, start, stop } = useTimeoutFn(() => {
    valueRef.value = null
  }, interval)

  const value = computed<UnwrapRef<T> | null>({
    get() {
      return valueRef.value
    },
    set(val: UnwrapRef<T> | null) {
      if (isPending) {
        stop()
      }
      if (val === null || !Number.isFinite(intervalRef.value)) {
        valueRef.value = val
      } else if (intervalRef.value > 0) {
        valueRef.value = val
        start()
      } else {
        valueRef.value = null
      }
    },
  })

  return { interval: intervalRef, value }
}
