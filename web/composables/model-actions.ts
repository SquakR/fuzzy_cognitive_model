import { useMessageStore } from '~/store'
import { ModelOutType } from '~/types'
import { MOVE_CONCEPT_KEY, ModelActionResult, MoveConceptType } from '~/types'

export const useModelActions = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => {
  const config = useRuntimeConfig()
  const messageStore = useMessageStore()

  const { execute: moveConceptExecute } = useMoveConcept({
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
    cy.value?.$(`#${getConceptId(concept.id)}`).position({
      x: concept.xPosition,
      y: concept.yPosition,
    })
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
      if (result.name === MOVE_CONCEPT_KEY) {
        moveConceptUpdate(result)
      }
    } catch {
      return
    }
  })

  onMounted(() => {
    open()
  })
  useEventListener(
    typeof window === 'undefined' ? null : window,
    'beforeunload',
    () => close()
  )

  return { moveConcept }
}
