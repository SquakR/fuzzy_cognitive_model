import colors from 'vuetify/lib/util/colors'
import {
  ConceptOutType,
  LocalFetchFuncOptions,
  ModelOutType,
  SET_IS_CONTROL_KEY,
  SetIsControlType,
  UseControlConceptsPlugin,
} from '~/types'

export const useControlConceptsPlugin: UseControlConceptsPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => {
  const IS_CONTROL_CLASS = 'is-control-concept'

  const isInstalled = computed(() =>
    model.value.project.plugins.includes('Control Concepts')
  )

  const getConceptClasses = (concept: ConceptOutType) => {
    if (
      concept.pluginsData.controlConcepts &&
      concept.pluginsData.controlConcepts.isControl
    ) {
      return [IS_CONTROL_CLASS]
    }
    return []
  }
  const getConnectionClasses = () => {
    return []
  }

  const getStyles = () => {
    return [
      {
        selector: `node:unselected.${IS_CONTROL_CLASS}`,
        style: {
          backgroundColor: colors.amber.lighten1,
        },
      },
    ]
  }

  const {
    execute: setIsControl,
    onSuccess: setIsControlOnSuccess,
    pending: setIsControlPending,
  } = useSetIsControl({
    key: SET_IS_CONTROL_KEY,
  })
  const setIsControlUpdate = (result: SetIsControlType) => {
    const concept = model.value.concepts.find(
      (concept) => concept.id === result.data.conceptId
    )!
    concept.pluginsData.controlConcepts!.isControl = result.data.isControl
    if (result.data.hasConstraint !== null) {
      concept.pluginsData.conceptConstraints!.hasConstraint =
        result.data.hasConstraint
    }
    concept.updatedAt = result.data.updatedAt
    const node = cy.value!.$(`#${getConceptId(concept.id)}`)
    if (result.data.isControl) {
      node.addClass(IS_CONTROL_CLASS)
    } else {
      node.removeClass(IS_CONTROL_CLASS)
    }
  }

  return {
    isInstalled,
    getConceptClasses,
    getConnectionClasses,
    getStyles,
    setIsControl,
    setIsControlOnSuccess,
    setIsControlPending,
    setIsControlUpdate,
  }
}

const useSetIsControl = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<SetIsControlType>(
    opts,
    {
      method: 'PATCH',
    }
  )
  const execute = async (conceptId: number, isControl: boolean) => {
    return await fetch(
      `/concepts/${conceptId}/change_is_control`,
      JSON.stringify(isControl)
    )
  }
  return { execute, ...rest }
}
