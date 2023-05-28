import cytoscape from 'cytoscape'
import colors from 'vuetify/lib/util/colors'
import { useUserStore } from '~/store'
import { ModelOutType } from '~/types'

export const useModel = (model: Ref<ModelOutType>) => {
  const NODE_HEIGHT = 50
  const NODE_WIDTH = 50
  const FONT_SIZE = 16
  const FONT_FAMILY = 'Roboto, sans-serif'
  const DESCRIPTION_MAX_WIDTH = 150
  const SELECTED_COLOR = colors.teal.lighten1

  const userStore = useUserStore()

  const container = ref<HTMLDivElement | null>(null)
  const cy = shallowRef<cytoscape.Core | null>(null)

  const plugins = usePlugins(model, cy)

  const createConceptElements = () =>
    model.value.concepts.map((concept) =>
      createConceptElement(model.value, concept, userStore.locale, plugins)
    )
  const createConnectionElements = () =>
    model.value.connections.map((connection) =>
      createConnectionElement(connection, userStore.locale, plugins)
    )

  const getConceptPositions = () =>
    model.value.concepts.reduce(
      (acc, concept) => ({
        ...acc,
        [getConceptId(concept)]: {
          x: concept.xPosition,
          y: concept.yPosition,
        },
      }),
      {}
    )

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
          selector: 'node:selected',
          style: {
            backgroundColor: SELECTED_COLOR,
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
      ],
    })
  }

  const listenRender = () => {
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
        const concept = model.value.concepts.find(
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
            if (
              line &&
              ctx.measureText(nextLine).width > DESCRIPTION_MAX_WIDTH
            ) {
              ctx.fillText(
                line,
                position.x - ctx.measureText(line).width / 2,
                y
              )
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

  watch(
    () => userStore.locale,
    async (newValue) => {
      cy.value!.$('node').forEach((node) => {
        const concept = model.value.concepts.find(
          (concept) => concept.id === node.data().conceptId
        )!
        node.data(createConceptData(model.value, concept, newValue))
      })
    }
  )

  onMounted(() => {
    cy.value = createCytoscape()
    listenRender()
  })

  return {
    container,
    cy,
    plugins,
  }
}
