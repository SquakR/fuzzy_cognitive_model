import { useAdjustmentPlugin } from './adjustment'
import { useConceptConstraintsPlugin } from './concept-constraints'
import { useConnectionConstraintsPlugin } from './connection-constraints'
import { useControlConceptsPlugin } from './control-concepts'
import { useControlConnectionsPlugin } from './control-connections'
import { useTargetConceptsPlugin } from './target-concepts'
import { ConceptOutType, ConnectionOutType, ModelOutType } from '~/types'
import { UsePlugins } from '~/types/plugins'

export const usePlugins: UsePlugins = (model: Ref<ModelOutType>) => {
  const controlConcepts = useControlConceptsPlugin(model)
  const targetConcepts = useTargetConceptsPlugin(model)
  const controlConnections = useControlConnectionsPlugin(model)
  const conceptConstraints = useConceptConstraintsPlugin(model)
  const connectionConstraints = useConnectionConstraintsPlugin(model)
  const adjustment = useAdjustmentPlugin(model)

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
