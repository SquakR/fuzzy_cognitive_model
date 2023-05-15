<template>
  <ModelEditorToolbar v-model:mode="mode" />
  <ModelAddConceptForm
    v-model="addConceptActive"
    :model="model"
    :x-position="addConceptXPosition"
    :y-position="addConceptYPosition"
    @create-concept="createConceptHandler"
  />
  <div ref="container" class="model-editor__cytoscape-container"></div>
</template>

<script setup lang="ts">
import { Mode } from './EditorToolbar.vue'
import cytoscape from 'cytoscape'
import colors from 'vuetify/lib/util/colors'
import { usePlugins } from '~/composables/plugins'
import { useUserStore } from '~/store'
import { ConceptInCreateType, ModelOutType } from '~/types'

export interface Props {
  model: ModelOutType
}
const props = defineProps<Props>()

const mode = ref<Mode>('change')

const addConceptActive = ref(false)
const addConceptXPosition = ref(0.0)
const addConceptYPosition = ref(0.0)

const container = ref<HTMLDivElement | null>(null)
const cy = shallowRef<cytoscape.Core | null>(null)

const userStore = useUserStore()

const plugins = usePlugins()

const { createConcept, createConceptOnSuccess, moveConcept } = useModelActions(
  toRef(props, 'model'),
  cy,
  plugins
)
createConceptOnSuccess(() => {
  addConceptActive.value = false
})
const createConceptHandler = (conceptIn: ConceptInCreateType) => {
  createConcept(props.model.project.id, conceptIn)
}

const getConceptElements = () =>
  props.model.concepts.map((concept) =>
    getConceptElement(props.model, concept, userStore.locale, plugins)
  )
const getConnectionElements = () =>
  props.model.connections.map((connection) => ({
    data: {
      connectionId: connection.id,
      id: getConnectionId(connection),
      source: getConceptId(connection.sourceId),
      target: getConceptId(connection.targetId),
      label: new Intl.NumberFormat(userStore.locale).format(connection.value),
    },
    classes: plugins.getConnectionClasses(connection).join(' '),
  }))

const getConceptPositions = () =>
  props.model.concepts.reduce(
    (acc, concept) => ({
      ...acc,
      [getConceptId(concept)]: {
        x: concept.xPosition,
        y: concept.yPosition,
      },
    }),
    {}
  )

onMounted(() => {
  cy.value = cytoscape({
    container: container.value,
    elements: [...getConceptElements(), ...getConnectionElements()],
    layout: {
      name: 'preset',
      fit: false,
      positions: getConceptPositions(),
    },
    style: [
      {
        selector: 'edge',
        style: {
          label: 'data(label)',
          width: 2,
          'line-color': colors.grey.darken3,
          'curve-style': 'straight',
          'target-arrow-color': colors.grey.darken3,
          'target-arrow-shape': 'triangle',
          'arrow-scale': 1.25,
          'text-background-opacity': 1,
          'text-background-color': colors.grey.lighten5,
        },
      },
      {
        selector: 'node',
        style: {
          label: 'data(label)',
          width: 50,
          height: 50,
          backgroundColor: colors.indigo.lighten1,
          'text-wrap': 'wrap',
          'text-margin-y': 75,
          'line-height':
            props.model.project.conceptValueType === 'none' ? 1.0 : 1.25,
        },
      },
      ...plugins.getStyles(),
    ],
  })
  cy.value.on('drag', 'node', async (e) => {
    const node = e.target
    const position = node.position()
    await moveConcept(node.data().conceptId, {
      xPosition: position.x,
      yPosition: position.y,
    })
  })
  cy.value.on('click', (e) => {
    if (mode.value === 'addConcept') {
      addConceptActive.value = true
      addConceptXPosition.value = e.position.x
      addConceptYPosition.value = e.position.y
    }
  })
})
</script>

<style lang="sass">
$grey-lighten-5: #FAFAFA

.model-editor__cytoscape-container
  width: 100%
  height: calc(100% - 48px)
  background-color: $grey-lighten-5
</style>
