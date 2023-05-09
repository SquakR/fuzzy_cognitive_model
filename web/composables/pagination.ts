import { AsyncDataExecuteOptions } from 'nuxt/dist/app/composables/asyncData'
import { PaginationOutType } from '~/types'

export const usePagination = <T>(
  pagination: Ref<PaginationOutType<T> | null>,
  refetch: (opts?: AsyncDataExecuteOptions | undefined) => Promise<void>,
  page: Ref<number>,
  perPage: Ref<number>
) => {
  const itemsLength = computed(() => {
    if (pagination.value) {
      return pagination.value.totalCount
    }
    return 0
  })
  const data = computed(() => {
    if (pagination.value) {
      return pagination.value.data
    }
  })

  const insertAtTop = async (data: T) => {
    if (pagination.value) {
      if (page.value === 1) {
        pagination.value.data.splice(0, 0, data)
        if (pagination.value.data.length > perPage.value) {
          pagination.value.data.splice(
            perPage.value,
            pagination.value.data.length - perPage.value
          )
        }
        pagination.value.totalCount += 1
        pagination.value.totalPages = Math.ceil(
          pagination.value.totalCount / perPage.value
        )
      } else {
        await refetch()
      }
    }
  }

  return { itemsLength, data, insertAtTop }
}
