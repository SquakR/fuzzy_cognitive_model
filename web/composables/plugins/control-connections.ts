import colors from 'vuetify/lib/util/colors'
import { ConnectionOutType } from '~/types'
import { UsePlugin } from '~/types/plugins'

export const useControlConnections: UsePlugin = () => {
  const getConceptClasses = () => {
    return []
  }
  const getConnectionClasses = (connection: ConnectionOutType) => {
    if (
      connection.pluginsData.controlConnections &&
      connection.pluginsData.controlConnections.isControl
    ) {
      return ['is-control-connection']
    }
    return []
  }
  const getStyles = () => {
    return [
      {
        selector: 'edge.is-control-connection',
        style: {
          'line-color': colors.amber.lighten1,
          'target-arrow-color': colors.amber.lighten1,
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
