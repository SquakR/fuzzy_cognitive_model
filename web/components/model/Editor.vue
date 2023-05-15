<template>
  <ModelEditorToolbar v-model:mode="mode" />
  <ModelAddConceptForm
    :model="model"
    :mode="mode"
    :cy="cy"
    :create-concept="createConcept"
    :create-concept-on-success="createConceptOnSuccess"
  />
  <ModelAddConnectionForm
    ref="modelAddConnectionForm"
    :model="model"
    :mode="mode"
    :cy="cy"
    :create-connection="createConnection"
    :create-connection-on-success="createConnectionOnSuccess"
  />
  <div ref="container" class="model-editor__cytoscape-container"></div>
</template>

<script setup lang="ts">
import cytoscape from 'cytoscape'
import colors from 'vuetify/lib/util/colors'
import ModelAddConnectionForm from '~/components/model/AddConnectionForm.vue'
import { usePlugins } from '~/composables/plugins'
import { useUserStore } from '~/store'
import { EditorMode, ModelOutType } from '~/types'

export interface Props {
  model: ModelOutType
}
const props = defineProps<Props>()

const modelAddConnectionForm = ref<InstanceType<
  typeof ModelAddConnectionForm
> | null>(null)

const mode = ref<EditorMode>('change')
watch(mode, (newValue, oldValue) => {
  if (newValue === 'change') {
    cy.value!.$('node').grabify()
  } else {
    cy.value!.$('node').ungrabify()
  }
  if (oldValue === 'addConnection') {
    modelAddConnectionForm.value!.clear()
  }
})

const container = ref<HTMLDivElement | null>(null)
const cy = shallowRef<cytoscape.Core | null>(null)

const userStore = useUserStore()
const plugins = usePlugins()

const {
  createConcept,
  createConceptOnSuccess,
  moveConcept,
  createConnection,
  createConnectionOnSuccess,
} = useModelActions(toRef(props, 'model'), cy, plugins)

const getConceptElements = () =>
  props.model.concepts.map((concept) =>
    getConceptElement(props.model, concept, userStore.locale, plugins)
  )
const getConnectionElements = () =>
  props.model.connections.map((connection) =>
    getConnectionElement(connection, userStore.locale, plugins)
  )

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

const NODE_HEIGHT = 50
const NODE_WIDTH = 50
const FONT_SIZE = 16
const FONT_FAMILY = 'Roboto, sans-serif'
const DESCRIPTION_MAX_WIDTH = 150

onMounted(() => {
  cy.value = createCytoscape()
  listen()
  drawLabels()
})

const createCytoscape = () => {
  return cytoscape({
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
          height: NODE_HEIGHT,
          width: NODE_WIDTH,
          backgroundColor: colors.indigo.lighten1,
          'text-valign': 'center',
          'text-margin-y': 2,
          'font-size': FONT_SIZE,
          'font-family': FONT_FAMILY,
        },
      },
      {
        selector: 'node.add-connection-source',
        style: {
          backgroundColor: colors.red.lighten1,
        },
      },
      {
        selector: 'node.add-connection-target',
        style: {
          backgroundColor: colors.purple.lighten1,
        },
      },
      ...plugins.getStyles(),
    ],
  })
}

const listen = () => {
  cy.value!.on('drag', 'node', async (e) => {
    if (mode.value !== 'change') {
      return
    }
    const node = e.target
    const position = node.position()
    await moveConcept(node.data().conceptId, {
      xPosition: position.x,
      yPosition: position.y,
    })
  })
}

const drawLabels = () => {
  const layer = cy.value!.cyCanvas()
  const canvas = layer.getCanvas()
  const ctx = canvas.getContext('2d')!
  cy.value!.on('render cyCanvas.resize', function () {
    ctx.font = `${FONT_SIZE}px ${FONT_FAMILY}`
    layer.resetTransform(ctx)
    layer.clear(ctx)
    layer.setTransform(ctx)
    cy.value!.nodes().forEach(function (node) {
      const position = node.position()
      const conceptId = node.data().conceptId
      const concept = props.model.concepts.find(
        (concept) => concept.id === conceptId
      )!
      ctx.fillText(
        concept.name,
        position.x - ctx.measureText(concept.name).width / 2,
        position.y - NODE_HEIGHT / 2 - 7
      )
      if (concept.description) {
        let y = position.y + NODE_HEIGHT / 2 + 17
        let line = ''
        for (const word of concept.description.split(' ')) {
          const nextLine = line ? `${line} ${word}` : word
          if (line && ctx.measureText(nextLine).width > DESCRIPTION_MAX_WIDTH) {
            ctx.fillText(line, position.x - ctx.measureText(line).width / 2, y)
            y += 16
            line = ''
          }
          line += ` ${word}`
        }
        if (line) {
          ctx.fillText(line, position.x - ctx.measureText(line).width / 2, y)
        }
      }
    })
  })
}
</script>

<style lang="sass">
$grey-lighten-5: #FAFAFA

.model-editor__cytoscape-container
  width: 100%
  height: calc(100% - 48px)
  background-color: $grey-lighten-5
</style>
