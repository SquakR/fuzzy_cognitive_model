import colors from 'vuetify/lib/util/colors'
import {
  ConnectionOutType,
  LocalFetchFuncOptions,
  ModelOutType,
  SET_IS_CONTROL_CONNECTION_KEY,
  SetIsControlConnectionType,
  UseControlConnectionsPlugin,
} from '~/types'

export const useControlConnectionsPlugin: UseControlConnectionsPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => {
  const IS_CONTROL_CLASS = 'is-control-connection'

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
      return [IS_CONTROL_CLASS]
    }
    return []
  }

  const getStyles = () => {
    return [
      {
        selector: `edge:unselected.${IS_CONTROL_CLASS}`,
        style: {
          'line-color': colors.amber.lighten1,
          'target-arrow-color': colors.amber.lighten1,
        },
      },
    ]
  }

  const {
    execute: setIsControl,
    onSuccess: setIsControlOnSuccess,
    pending: setIsControlPending,
  } = useSetIsControl({ key: SET_IS_CONTROL_CONNECTION_KEY })
  const setIsControlUpdate = (result: SetIsControlConnectionType) => {
    const connection = model.value.connections.find(
      (connection) => connection.id === result.data.connectionId
    )!
    connection.pluginsData.controlConnections!.isControl = result.data.isControl
    if (result.data.hasConstraint !== null) {
      connection.pluginsData.connectionConstraints!.hasConstraint =
        result.data.hasConstraint
    }
    connection.updatedAt = result.data.updatedAt
    const edge = cy.value!.$(`#${getConnectionId(connection.id)}`)
    if (result.data.isControl) {
      edge.addClass(IS_CONTROL_CLASS)
    } else {
      edge.removeClass(IS_CONTROL_CLASS)
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
  const { execute: fetch, ...rest } =
    useLocalFetchFunc<SetIsControlConnectionType>(opts, {
      method: 'PATCH',
    })
  const execute = async (connectionId: number, isControl: boolean) => {
    return await fetch(
      `/connection/${connectionId}/change_is_control`,
      JSON.stringify(isControl)
    )
  }
  return { execute, ...rest }
}
