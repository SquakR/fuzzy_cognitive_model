import { UseFetchOptions } from 'nuxt/app'
import { useMessageStore } from '~/store'
import {
  ADJUSTMENT_GENERATION_KEY,
  ADJUSTMENT_RESULT_KEY,
  ADJUST_KEY,
  AdjustType,
  AdjustmentGenerationOutType,
  AdjustmentGenerationType,
  AdjustmentInType,
  AdjustmentIndividualOutType,
  AdjustmentResultType,
  AdjustmentRunActionResult,
  AdjustmentRunOutType,
  AdjustmentRunsInType,
  CHANGE_DYNAMIC_MODEL_TYPE_KEY,
  ChangeDynamicModelTypeType,
  DynamicModelType,
  LocalFetchFuncOptions,
  LocalFetchOptions,
  ModelOutType,
  PaginationOutType,
  UseAdjustmentPlugin,
} from '~/types'

export const useAdjustmentPlugin: UseAdjustmentPlugin = (
  model: Ref<ModelOutType>
) => {
  const isInstalled = computed(() =>
    model.value.project.plugins.includes('Adjustment With Genetic Algorithms')
  )

  const getConceptClasses = () => {
    return []
  }
  const getConnectionClasses = () => {
    return []
  }

  const getStyles = () => {
    return []
  }

  const {
    execute: changeDynamicModelType,
    onSuccess: changeDynamicModelTypeOnSuccess,
    pending: changeDynamicModelTypePending,
  } = useChangeDynamicModelType({
    key: CHANGE_DYNAMIC_MODEL_TYPE_KEY,
  })
  const changeDynamicModelTypeUpdate = (result: ChangeDynamicModelTypeType) => {
    const concept = model.value.concepts.find(
      (concept) => concept.id === result.data.conceptId
    )!
    concept.pluginsData.adjustment!.dynamicModelType =
      result.data.dynamicModelType
    concept.updatedAt = result.data.updatedAt
  }

  return {
    isInstalled,
    getConceptClasses,
    getConnectionClasses,
    getStyles,
    changeDynamicModelType,
    changeDynamicModelTypeOnSuccess,
    changeDynamicModelTypePending,
    changeDynamicModelTypeUpdate,
  }
}

const useChangeDynamicModelType = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } =
    useLocalFetchFunc<ChangeDynamicModelTypeType>(opts, {
      method: 'PATCH',
    })
  const execute = async (
    conceptId: number,
    dynamicModelType: DynamicModelType | null
  ) => {
    return await fetch(
      `/concepts/${conceptId}/change_dynamic_model_type`,
      JSON.stringify(dynamicModelType)
    )
  }
  return { execute, ...rest }
}

export type LocalAdjustmentRunsInType = AdjustmentRunsInType & {
  page: number
  perPage: number
}

export const useAdjustmentRuns = async (
  projectId: number,
  adjustmentRunsIn: Ref<LocalAdjustmentRunsInType>
) => {
  const config = useRuntimeConfig()
  const messageStore = useMessageStore()

  const {
    execute: adjust,
    onSuccess: adjustOnSuccess,
    pending: adjustPending,
  } = useAdjust({
    key: ADJUST_KEY,
  })

  const adjustUpdate = (result: AdjustType) => {
    insertAtTop(result.data)
  }
  const adjustmentResultUpdate = (result: AdjustmentResultType) => {
    replace(result.data)
  }
  const adjustmentGenerationUpdate = (result: AdjustmentGenerationType) => {
    if (lastGenerations.value[result.adjustmentRunId] !== undefined) {
      lastGenerations.value[result.adjustmentRunId] = result.data.number
    }
  }

  const { data, open, close } = useWebSocket<string>(
    `${config.public.API_WS_BASE_URL}/adjustment_run/${projectId}`,
    {
      autoReconnect: true,
      heartbeat: true,
      immediate: false,
      autoClose: false,
    }
  )
  watch(data, (newValue) => {
    if (newValue === null) {
      return
    }
    let result: AdjustmentRunActionResult
    try {
      result = JSON.parse(newValue)
      if (!('data' in result)) {
        messageStore.emitError(result.name, result.message)
        return
      }
      switch (result.name) {
        case ADJUST_KEY:
          adjustUpdate(result)
          break
        case ADJUSTMENT_RESULT_KEY:
          adjustmentResultUpdate(result)
          break
        case ADJUSTMENT_GENERATION_KEY:
          adjustmentGenerationUpdate(result)
          break
      }
    } catch {
      return
    }
  })

  onMounted(() => {
    open()
  })
  onUnmounted(() => close())

  const {
    data: adjustmentRunsPagination,
    pending: adjustmentRunsPending,
    refresh,
  } = await useGetAdjustmentRuns(
    { key: 'adjustmentRuns', fatal: false },
    projectId,
    adjustmentRunsIn
  )

  const {
    itemsLength: adjustmentRunsItemsLength,
    data: adjustmentRuns,
    insertAtTop,
    replace,
  } = usePagination(
    adjustmentRunsPagination,
    refresh,
    toRef(adjustmentRunsIn.value, 'page'),
    toRef(adjustmentRunsIn.value, 'perPage')
  )

  const lastGenerations = ref<Record<number, number>>({})
  watch(
    adjustmentRuns,
    (newValue) => {
      if (newValue) {
        for (const adjustmentRun of newValue) {
          if (adjustmentRun.resultIndividual) {
            delete lastGenerations.value[adjustmentRun.id]
          } else if (!lastGenerations.value[adjustmentRun.id]) {
            lastGenerations.value[adjustmentRun.id] = 0
          }
        }
      } else {
        lastGenerations.value = {}
      }
    },
    { immediate: true, deep: true }
  )

  return {
    adjustmentRuns,
    adjustmentRunsPending,
    adjustmentRunsItemsLength,
    lastGenerations,
    adjust,
    adjustOnSuccess,
    adjustPending,
  }
}

const useAdjust = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<AdjustType>(opts, {
    method: 'POST',
  })
  const execute = async (projectId: number, adjustmentIn: AdjustmentInType) => {
    return await fetch(`/projects/${projectId}/adjust`, adjustmentIn)
  }
  return { execute, ...rest }
}

export const useGetAdjustmentRun = (
  opts: LocalFetchOptions,
  adjustmentRunId: number,
  fetchOptions?: UseFetchOptions<AdjustmentRunOutType>
) => {
  return useLocalFetch<AdjustmentRunOutType>(
    `/adjustment_runs/${adjustmentRunId}`,
    opts,
    {
      ...fetchOptions,
      method: 'GET',
    }
  )
}

const useGetAdjustmentRuns = (
  opts: LocalFetchOptions,
  projectId: number,
  adjustmentRunsIn: Ref<AdjustmentRunsInType>,
  fetchOptions?: UseFetchOptions<PaginationOutType<AdjustmentRunOutType>>
) => {
  return useLocalFetch<PaginationOutType<AdjustmentRunOutType>>(
    `/projects/${projectId}/adjustment_runs`,
    opts,
    {
      ...fetchOptions,
      method: 'GET',
      params: computed(() =>
        Object.fromEntries(
          Object.entries(adjustmentRunsIn.value).filter(([_, v]) => !!v)
        )
      ),
    }
  )
}

export const useGetAdjustmentGeneration = (
  opts: LocalFetchOptions,
  adjustmentGenerationId: number,
  fetchOptions?: UseFetchOptions<AdjustmentGenerationOutType>
) => {
  return useLocalFetch<AdjustmentGenerationOutType>(
    `/adjustment_generations/${adjustmentGenerationId}`,
    opts,
    {
      ...fetchOptions,
      method: 'GET',
    }
  )
}

export const useGetAdjustmentGenerations = (
  opts: LocalFetchOptions,
  adjustmentRunId: number,
  page: Ref<number>,
  perPage: Ref<number>,
  fetchOptions?: UseFetchOptions<PaginationOutType<AdjustmentGenerationOutType>>
) => {
  return useLocalFetch<PaginationOutType<AdjustmentGenerationOutType>>(
    `/adjustment_runs/${adjustmentRunId}/adjustment_generations`,
    opts,
    {
      ...fetchOptions,
      method: 'GET',
      params: computed(() => ({
        page: page.value,
        perPage: perPage.value,
      })),
    }
  )
}

export const useGetAdjustmentIndividual = (
  opts: LocalFetchOptions,
  adjustmentIndividualId: number,
  fetchOptions?: UseFetchOptions<AdjustmentIndividualOutType>
) => {
  return useLocalFetch<AdjustmentIndividualOutType>(
    `/adjustment_individuals/${adjustmentIndividualId}`,
    opts,
    {
      ...fetchOptions,
      method: 'GET',
    }
  )
}

export const useGetAdjustmentIndividuals = (
  opts: LocalFetchOptions,
  adjustmentGenerationId: number,
  page: Ref<number>,
  perPage: Ref<number>,
  fetchOptions?: UseFetchOptions<PaginationOutType<AdjustmentIndividualOutType>>
) => {
  return useLocalFetch<PaginationOutType<AdjustmentIndividualOutType>>(
    `/adjustment_generations/${adjustmentGenerationId}/adjustment_individuals`,
    opts,
    {
      ...fetchOptions,
      method: 'GET',
      params: computed(() => ({
        page: page.value,
        perPage: perPage.value,
      })),
    }
  )
}
