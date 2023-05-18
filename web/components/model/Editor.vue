<template>
  <ModelEditorToolbar v-model:mode="mode" />
  <ModelAddConceptForm
    v-if="cy"
    :model="model"
    :mode="mode"
    :cy="cy"
    :create-concept="createConcept"
    :create-concept-on-success="createConceptOnSuccess"
  />
  <ModelAddConnectionForm
    v-if="cy"
    ref="modelAddConnectionForm"
    :model="model"
    :mode="mode"
    :cy="cy"
    :create-connection="createConnection"
    :create-connection-on-success="createConnectionOnSuccess"
  />
  <ModelChangeConceptDrawer
    v-if="cy"
    :model="model"
    :plugins="plugins"
    :mode="mode"
    :cy="cy"
    :change-concept="changeConcept"
    :change-concept-on-success="changeConceptOnSuccess"
  />
  <div ref="container" class="model-editor__cytoscape-container" />
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
    cy.value!.$('node').selectify().grabify()
    cy.value!.$('edge').selectify()
  } else {
    cy.value!.$('node').unselect().unselectify().ungrabify()
    cy.value!.$('edge').unselect().unselectify()
  }
  if (oldValue === 'addConnection') {
    modelAddConnectionForm.value!.clear()
  }
})

const container = ref<HTMLDivElement | null>(null)
const cy = shallowRef<cytoscape.Core | null>(null)

const userStore = useUserStore()
watch(
  () => userStore.locale,
  async (newValue) => {
    cy.value!.$('node').forEach((node) => {
      const concept = props.model.concepts.find(
        (concept) => concept.id === node.data().conceptId
      )!
      node.data(createConceptData(props.model, concept, newValue))
    })
  }
)

const plugins = usePlugins(toRef(props, 'model'))

const {
  createConcept,
  createConceptOnSuccess,
  changeConcept,
  changeConceptOnSuccess,
  moveConcept,
  deleteConcept,
  createConnection,
  createConnectionOnSuccess,
  deleteConnection,
} = useModelActions(toRef(props, 'model'), cy, plugins)

const createConceptElements = () =>
  props.model.concepts.map((concept) =>
    createConceptElement(props.model, concept, userStore.locale, plugins)
  )
const createConnectionElements = () =>
  props.model.connections.map((connection) =>
    createConnectionElement(connection, userStore.locale, plugins)
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
const SELECTED_COLOR = colors.teal.lighten1

onMounted(() => {
  cy.value = createCytoscape()
  listen()
  drawLabels()
})

onKeyStroke('Delete', () => {
  cy.value!.$('node:selected').forEach((node) => {
    const conceptId = node.data().conceptId
    deleteConcept(conceptId)
  })
  cy.value!.$('edge:selected').forEach((edge) => {
    const connectionId = edge.data().connectionId
    deleteConnection(connectionId)
  })
})

const createCytoscape = () => {
  return cytoscape({
    container: container.value,
    elements: [...createConceptElements(), ...createConnectionElements()],
    layout: {
      name: 'preset',
      fit: false,
      positions: getConceptPositions(),
    },
    style: [
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
        selector: 'node:selected',
        style: {
          backgroundColor: SELECTED_COLOR,
        },
      },
      {
        selector: 'node.add-connection-source',
        style: {
          backgroundColor: colors.teal.lighten1,
        },
      },
      {
        selector: 'node.add-connection-target',
        style: {
          backgroundColor: colors.purple.lighten1,
        },
      },
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
        selector: 'edge:selected',
        style: {
          'line-color': SELECTED_COLOR,
          'target-arrow-color': SELECTED_COLOR,
        },
      },
      ...plugins.getStyles(),
    ],
  })
}

const listen = () => {
  cy.value!.on('select', 'node, edge', (e) =>
    cy.value!.elements().not(e.target).unselect()
  )
  cy.value!.on('drag', 'node', async (e) => {
    if (mode.value !== 'change') {
      return
    }
    cy.value!.elements().not(e.target).unselect()
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
