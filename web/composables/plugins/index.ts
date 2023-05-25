import { useAdjustmentPlugin } from './adjustment'
import { useConceptConstraintsPlugin } from './concept-constraints'
import { useConnectionConstraintsPlugin } from './connection-constraints'
import { useControlConceptsPlugin } from './control-concepts'
import { useControlConnectionsPlugin } from './control-connections'
import { useTargetConceptsPlugin } from './target-concepts'
import { ConceptOutType, ConnectionOutType, ModelOutType } from '~/types'

export { useAdjustmentRuns } from './adjustment'

export const usePlugins = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => {
  const controlConcepts = useControlConceptsPlugin(model, cy)
  const targetConcepts = useTargetConceptsPlugin(model, cy)
  const controlConnections = useControlConnectionsPlugin(model, cy)
  const conceptConstraints = useConceptConstraintsPlugin(model, cy)
  const connectionConstraints = useConnectionConstraintsPlugin(model, cy)
  const adjustment = useAdjustmentPlugin(model, cy)

  const plugins = [
    controlConcepts,
    targetConcepts,
    controlConnections,
    conceptConstraints,
    connectionConstraints,
    adjustment,
  ]

  const getConceptClasses = (concept: ConceptOutType) => {
    let classes: string[] = []
    for (const plugin of plugins) {
      classes = [...classes, ...plugin.getConceptClasses(concept)]
    }
    return classes
  }
  const getConnectionClasses = (connection: ConnectionOutType) => {
    let classes: string[] = []
    for (const plugin of plugins) {
      classes = [...classes, ...plugin.getConnectionClasses(connection)]
    }
    return classes
  }
  const getStyles = () => {
    let styles: cytoscape.Stylesheet[] = []
    for (const plugin of plugins) {
      styles = [...styles, ...plugin.getStyles()]
    }
    return styles
  }

  return {
    controlConcepts,
    targetConcepts,
    controlConnections,
    conceptConstraints,
    connectionConstraints,
    adjustment,
    getConceptClasses,
    getConnectionClasses,
    getStyles,
  }
}
