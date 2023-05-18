import colors from 'vuetify/lib/util/colors'
import { ConceptOutType, ModelOutType } from '~/types'
import { UsePlugin } from '~/types/plugins'

export const useTargetConceptsPlugin: UsePlugin = (
  model: Ref<ModelOutType>
) => {
  const isInstalled = computed(() =>
    model.value.project.plugins.includes('Target Concepts')
  )

  const getConceptClasses = (concept: ConceptOutType) => {
    if (
      concept.pluginsData.targetConcepts &&
      concept.pluginsData.targetConcepts.isTarget
    ) {
      return ['is-target-concept']
    }
    return []
  }
  const getConnectionClasses = () => {
    return []
  }

  const getStyles = () => {
    return [
      {
        selector: 'node.is-target-concept',
        style: {
          backgroundColor: colors.lime.lighten1,
        },
      },
    ]
  }

  return {
    isInstalled,
    getConceptClasses,
    getConnectionClasses,
    getStyles,
  }
}
