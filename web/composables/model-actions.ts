import { useMessageStore, useUserStore } from '~/store'
import {
  CHANGE_CONCEPT_CONSTRAINT_KEY,
  CHANGE_CONCEPT_KEY,
  CHANGE_CONNECTION_KEY,
  CHANGE_DYNAMIC_MODEL_TYPE_KEY,
  CHANGE_TARGET_CONCEPT_KEY,
  CREATE_CONCEPT_KEY,
  CREATE_CONNECTION_KEY,
  ChangeConceptType,
  ChangeConnectionType,
  CreateConceptType,
  CreateConnectionType,
  DELETE_CONCEPT_KEY,
  DELETE_CONNECTION_KEY,
  DeleteConceptType,
  DeleteConnectionType,
  EditorMode,
  ModelOutType,
  Plugins,
  SET_IS_CONTROL_KEY,
} from '~/types'
import { MOVE_CONCEPT_KEY, ModelActionResult, MoveConceptType } from '~/types'

export const useModelActions = (
  model: Ref<ModelOutType>,
  mode: Ref<EditorMode>,
  cy: Ref<cytoscape.Core | null>,
  plugins: Plugins
) => {
  const config = useRuntimeConfig()
  const userStore = useUserStore()
  const messageStore = useMessageStore()

  const {
    execute: createConcept,
    onSuccess: createConceptOnSuccess,
    pending: createConceptPending,
  } = useCreateConcept({
    key: CREATE_CONCEPT_KEY,
  })
  const createConceptUpdate = (result: CreateConceptType) => {
    model.value.concepts.push(result.data)
    cy.value!.add({
      ...createConceptElement(
        model.value,
        result.data,
        userStore.locale,
        plugins
      ),
      selectable: mode.value === 'change',
      grabbable: mode.value === 'change',
    })
    setConceptPosition(cy.value!, model.value.concepts.at(-1)!)
  }

  const {
    execute: changeConcept,
    onSuccess: changeConceptOnSuccess,
    pending: changeConceptPending,
  } = useChangeConcept({ key: CHANGE_CONCEPT_KEY })
  const changeConceptUpdate = (result: ChangeConceptType) => {
    const concept = model.value.concepts.find(
      (concept) => concept.id === result.data.id
    )!
    concept.name = result.data.name
    concept.description = result.data.description
    concept.value = result.data.value
    concept.xPosition = result.data.xPosition
    concept.yPosition = result.data.yPosition
    concept.updatedAt = result.data.updatedAt
    setConceptDataWithPosition(
      cy.value!,
      model.value,
      concept,
      userStore.locale
    )
  }

  const {
    execute: moveConceptExecute,
    onSuccess: moveConceptOnSuccess,
    pending: moveConceptPending,
  } = useMoveConcept({
    key: MOVE_CONCEPT_KEY,
  })
  const moveConcept = useDebounceFn(moveConceptExecute, 500)
  const moveConceptUpdate = (result: MoveConceptType) => {
    const concept = model.value.concepts.find(
      (concept) => concept.id === result.data.id
    )!
    concept.xPosition = result.data.xPosition
    concept.yPosition = result.data.yPosition
    concept.updatedAt = result.data.updatedAt
    setConceptPosition(cy.value!, concept)
  }

  const {
    execute: deleteConcept,
    onSuccess: deleteConceptOnSuccess,
    pending: deleteConceptPending,
  } = useDeleteConcept({
    key: DELETE_CONCEPT_KEY,
  })
  const deleteConceptUpdate = (result: DeleteConceptType) => {
    model.value.concepts = model.value.concepts.filter(
      (concept) => concept.id !== result.data.id
    )
    cy.value!.$(`#${getConceptId(result.data.id)}`).remove()
  }

  const {
    execute: createConnection,
    onSuccess: createConnectionOnSuccess,
    pending: createConnectionPending,
  } = useCreateConnection({ key: CREATE_CONNECTION_KEY })
  const createConnectionUpdate = (result: CreateConnectionType) => {
    model.value.connections.push(result.data)
    cy.value!.add({
      ...createConnectionElement(result.data, userStore.locale, plugins),
      selectable: mode.value === 'change',
    })
  }

  const {
    execute: changeConnection,
    onSuccess: changeConnectionOnSuccess,
    pending: changeConnectionPending,
  } = useChangeConnection({ key: CHANGE_CONNECTION_KEY })
  const changeConnectionUpdate = (result: ChangeConnectionType) => {
    const connection = model.value.connections.find(
      (connection) => connection.id === result.data.id
    )!
    connection.description = result.data.description
    connection.value = result.data.value
    connection.updatedAt = result.data.updatedAt
    setConnectionData(cy.value!, connection, userStore.locale)
  }

  const {
    execute: deleteConnection,
    onSuccess: deleteConnectionOnSuccess,
    pending: deleteConnectionPending,
  } = useDeleteConnection({ key: DELETE_CONNECTION_KEY })
  const deleteConnectionUpdate = (result: DeleteConnectionType) => {
    model.value.connections = model.value.connections.filter(
      (connection) => connection.id !== result.data.id
    )
    cy.value!.$(`#${getConnectionId(result.data.id)}`).remove()
  }

  const { data, open, close } = useWebSocket<string>(
    `${config.public.API_WS_BASE_URL}/project/${model.value.project.id}`,
    {
      autoReconnect: true,
      heartbeat: true,
      immediate: false,
      autoClose: false,
    }
  )
  watch(data, (newValue) => {
    if (newValue === null) {
      return
    }
    let result: ModelActionResult
    try {
      result = JSON.parse(newValue)
      if (!('data' in result)) {
        messageStore.emitError(result.name, result.message)
        return
      }
      model.value.project.updatedAt = result.projectUpdatedAt
      switch (result.name) {
        case CREATE_CONCEPT_KEY:
          createConceptUpdate(result)
          break
        case CHANGE_CONCEPT_KEY:
          changeConceptUpdate(result)
          break
        case MOVE_CONCEPT_KEY:
          moveConceptUpdate(result)
          break
        case DELETE_CONCEPT_KEY:
          deleteConceptUpdate(result)
          break
        case CREATE_CONNECTION_KEY:
          createConnectionUpdate(result)
          break
        case CHANGE_CONNECTION_KEY:
          changeConnectionUpdate(result)
          break
        case DELETE_CONNECTION_KEY:
          deleteConnectionUpdate(result)
          break
        case SET_IS_CONTROL_KEY:
          plugins.controlConcepts.setIsControlUpdate(result)
          break
        case CHANGE_TARGET_CONCEPT_KEY:
          plugins.targetConcepts.changeTargetConceptUpdate(result)
          break
        case CHANGE_CONCEPT_CONSTRAINT_KEY:
          plugins.conceptConstraints.changeConceptConstraintUpdate(result)
          break
        case CHANGE_DYNAMIC_MODEL_TYPE_KEY:
          plugins.adjustment.changeDynamicModelTypeUpdate(result)
          break
      }
    } catch {
      return
    }
  })

  onMounted(() => {
    open()
  })
  onUnmounted(() => close())

  return {
    createConcept,
    createConceptOnSuccess,
    createConceptPending,
    changeConcept,
    changeConceptOnSuccess,
    changeConceptPending,
    moveConcept,
    moveConceptOnSuccess,
    moveConceptPending,
    deleteConcept,
    deleteConceptOnSuccess,
    deleteConceptPending,
    createConnection,
    createConnectionOnSuccess,
    createConnectionPending,
    changeConnection,
    changeConnectionOnSuccess,
    changeConnectionPending,
    deleteConnection,
    deleteConnectionOnSuccess,
    deleteConnectionPending,
  }
}
