import {
  CHANGE_DYNAMIC_MODEL_TYPE_KEY,
  ChangeDynamicModelTypeType,
  DynamicModelType,
  LocalFetchFuncOptions,
  ModelOutType,
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
