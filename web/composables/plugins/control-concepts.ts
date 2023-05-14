import colors from 'vuetify/lib/util/colors'
import { ConceptOutType } from '~/types'
import { UsePlugin } from '~/types/plugins'

export const useControlConcepts: UsePlugin = () => {
  const getConceptClasses = (concept: ConceptOutType) => {
    if (
      concept.pluginsData.controlConcepts &&
      concept.pluginsData.controlConcepts.isControl
    ) {
      return ['is-control-concept']
    }
    return []
  }
  const getConnectionClasses = () => {
    return []
  }
  const getStyles = () => {
    return [
      {
        selector: 'node.is-control-concept',
        style: {
          backgroundColor: colors.amber.lighten1,
        },
      },
    ]
  }

  return {
    getConceptClasses,
    getConnectionClasses,
    getStyles,
  }
}
