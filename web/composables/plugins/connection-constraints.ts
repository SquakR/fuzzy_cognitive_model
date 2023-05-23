import {
  CHANGE_CONNECTION_CONSTRAINT_KEY,
  ChangeConnectionConstraintType,
  ConnectionConstraintInChangeType,
  LocalFetchFuncOptions,
  ModelOutType,
  UseConnectionConstraintsPlugin,
} from '~/types'

export const useConnectionConstraintsPlugin: UseConnectionConstraintsPlugin = (
  model: Ref<ModelOutType>
) => {
  const isInstalled = computed(() =>
    model.value.project.plugins.includes('Connection Constraints')
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
    execute: changeConnectionConstraint,
    onSuccess: changeConnectionConstraintOnSuccess,
    pending: changeConnectionConstraintPending,
  } = useChangeConnectionConstraint({
    key: CHANGE_CONNECTION_CONSTRAINT_KEY,
  })
  const changeConnectionConstraintUpdate = (
    result: ChangeConnectionConstraintType
  ) => {
    const connection = model.value.connections.find(
      (connection) => connection.id === result.data.connectionId
    )!
    connection.pluginsData.connectionConstraints!.hasConstraint =
      result.data.hasConstraint
    connection.pluginsData.connectionConstraints!.minValue =
      result.data.minValue
    connection.pluginsData.connectionConstraints!.includeMinValue =
      result.data.includeMinValue
    connection.pluginsData.connectionConstraints!.maxValue =
      result.data.maxValue
    connection.pluginsData.connectionConstraints!.includeMaxValue =
      result.data.includeMaxValue
    connection.updatedAt = result.data.updatedAt
  }

  return {
    isInstalled,
    getConceptClasses,
    getConnectionClasses,
    getStyles,
    changeConnectionConstraint,
    changeConnectionConstraintOnSuccess,
    changeConnectionConstraintPending,
    changeConnectionConstraintUpdate,
  }
}

const useChangeConnectionConstraint = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } =
    useLocalFetchFunc<ChangeConnectionConstraintType>(opts, {
      method: 'PATCH',
    })
  const execute = async (
    connectionId: number,
    connectionConstraintIn: ConnectionConstraintInChangeType
  ) => {
    return await fetch(
      `/connections/${connectionId}/change_connection_constraint`,
      connectionConstraintIn
    )
  }
  return { execute, ...rest }
}
