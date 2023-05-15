import { useMessageStore, useUserStore } from '~/store'
import {
  CREATE_CONCEPT_KEY,
  CreateConceptType,
  ModelOutType,
  Plugin,
} from '~/types'
import { MOVE_CONCEPT_KEY, ModelActionResult, MoveConceptType } from '~/types'

export const useModelActions = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>,
  plugins: Plugin
) => {
  const config = useRuntimeConfig()
  const userStore = useUserStore()
  const messageStore = useMessageStore()

  const { execute: createConcept, onSuccess: createConceptOnSuccess } =
    useCreateConcept({
      key: CREATE_CONCEPT_KEY,
    })
  const createConceptUpdate = (result: CreateConceptType) => {
    model.value.concepts.push(result.data)
    cy.value?.add(
      getConceptElement(model.value, result.data, userStore.locale, plugins)
    )
    if (cy.value) {
      setConceptPosition(cy.value, model.value.concepts.at(-1)!)
    }
  }

  const { execute: moveConceptExecute, onSuccess: moveConceptOnSuccess } =
    useMoveConcept({
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
    if (cy.value) {
      setConceptPosition(cy.value, concept)
    }
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
        case MOVE_CONCEPT_KEY:
          moveConceptUpdate(result)
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
    moveConcept,
    moveConceptOnSuccess,
  }
}
