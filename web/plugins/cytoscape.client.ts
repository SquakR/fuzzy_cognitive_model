import cytoscape from 'cytoscape'
import cytoscapeCanvas from 'cytoscape-canvas'

export default defineNuxtPlugin(() => {
  cytoscapeCanvas(cytoscape)
})
