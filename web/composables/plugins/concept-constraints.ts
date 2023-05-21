import {
  CHANGE_CONCEPT_CONSTRAINT_KEY,
  ChangeConceptConstraintType,
  ConceptConstraintInChangeType,
  LocalFetchFuncOptions,
  ModelOutType,
  UseConceptConstraintsPlugin,
} from '~/types'

export const useConceptConstraintsPlugin: UseConceptConstraintsPlugin = (
  model: Ref<ModelOutType>
) => {
  const isInstalled = computed(() =>
    model.value.project.plugins.includes('Concept Constraints')
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
    execute: changeConceptConstraint,
    onSuccess: changeConceptConstraintOnSuccess,
    pending: changeConceptConstraintPending,
  } = useChangeConceptConstraint({
    key: CHANGE_CONCEPT_CONSTRAINT_KEY,
  })
  const changeConceptConstraintUpdate = (
    result: ChangeConceptConstraintType
  ) => {
    const concept = model.value.concepts.find(
      (concept) => concept.id === result.data.conceptId
    )!
    concept.pluginsData.conceptConstraints!.hasConstraint =
      result.data.hasConstraint
    concept.pluginsData.conceptConstraints!.minValue = result.data.minValue
    concept.pluginsData.conceptConstraints!.includeMinValue =
      result.data.includeMinValue
    concept.pluginsData.conceptConstraints!.maxValue = result.data.maxValue
    concept.pluginsData.conceptConstraints!.includeMaxValue =
      result.data.includeMaxValue
    concept.updatedAt = result.data.updatedAt
  }

  return {
    isInstalled,
    getConceptClasses,
    getConnectionClasses,
    getStyles,
    changeConceptConstraint,
    changeConceptConstraintOnSuccess,
    changeConceptConstraintPending,
    changeConceptConstraintUpdate,
  }
}

const useChangeConceptConstraint = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } =
    useLocalFetchFunc<ChangeConceptConstraintType>(opts, {
      method: 'PATCH',
    })
  const execute = async (
    conceptId: number,
    conceptConstraintIn: ConceptConstraintInChangeType
  ) => {
    return await fetch(
      `/concepts/${conceptId}/change_concept_constraint`,
      conceptConstraintIn
    )
  }
  return { execute, ...rest }
}
