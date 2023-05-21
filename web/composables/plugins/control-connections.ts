import colors from 'vuetify/lib/util/colors'
import {
  ConnectionOutType,
  ModelOutType,
  UseControlConnectionsPlugin,
} from '~/types'

export const useControlConnectionsPlugin: UseControlConnectionsPlugin = (
  model: Ref<ModelOutType>
) => {
  const isInstalled = computed(() =>
    model.value.project.plugins.includes('Control Connections')
  )

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
        selector: 'edge:unselected.is-control-connection',
        style: {
          'line-color': colors.amber.lighten1,
          'target-arrow-color': colors.amber.lighten1,
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
