import colors from 'vuetify/lib/util/colors'
import {
  CHANGE_TARGET_CONCEPT_KEY,
  ChangeTargetConceptType,
  ConceptOutType,
  LocalFetchFuncOptions,
  ModelOutType,
  TargetConceptInChangeType,
  UseTargetConceptsPlugin,
} from '~/types'

export const useTargetConceptsPlugin: UseTargetConceptsPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => {
  const IS_TARGET_CLASS = 'is-target-concept'

  const isInstalled = computed(() =>
    model.value.project.plugins.includes('Target Concepts')
  )

  const getConceptClasses = (concept: ConceptOutType) => {
    if (
      concept.pluginsData.targetConcepts &&
      concept.pluginsData.targetConcepts.isTarget
    ) {
      return [IS_TARGET_CLASS]
    }
    return []
  }
  const getConnectionClasses = () => {
    return []
  }

  const getStyles = () => {
    return [
      {
        selector: `node:unselected.${IS_TARGET_CLASS}`,
        style: {
          backgroundColor: colors.lime.lighten1,
        },
      },
    ]
  }

  const {
    execute: changeTargetConcept,
    onSuccess: changeTargetConceptOnSuccess,
    pending: changeTargetConceptPending,
  } = useChangeTargetConcept({
    key: CHANGE_TARGET_CONCEPT_KEY,
  })
  const changeTargetConceptUpdate = (result: ChangeTargetConceptType) => {
    const concept = model.value.concepts.find(
      (concept) => concept.id === result.data.conceptId
    )!
    concept.pluginsData.targetConcepts!.isTarget = result.data.isTarget
    concept.pluginsData.targetConcepts!.value = result.data.value
    concept.updatedAt = result.data.updatedAt
    const node = cy.value!.$(`#${getConceptId(concept.id)}`)
    if (result.data.isTarget) {
      node.addClass(IS_TARGET_CLASS)
    } else {
      node.removeClass(IS_TARGET_CLASS)
    }
  }

  return {
    isInstalled,
    getConceptClasses,
    getConnectionClasses,
    getStyles,
    changeTargetConcept,
    changeTargetConceptOnSuccess,
    changeTargetConceptPending,
    changeTargetConceptUpdate,
  }
}

const useChangeTargetConcept = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } =
    useLocalFetchFunc<ChangeTargetConceptType>(opts, {
      method: 'PATCH',
    })
  const execute = async (
    conceptId: number,
    targetConceptIn: TargetConceptInChangeType
  ) => {
    return await fetch(
      `/concepts/${conceptId}/change_target_concept`,
      targetConceptIn
    )
  }
  return { execute, ...rest }
}
