import { useControlConcepts } from './control-concepts'
import { useControlConnections } from './control-connections'
import { useTargetConcepts } from './target-concepts'
import { ConceptOutType, ConnectionOutType } from '~/types'
import { UsePlugin } from '~/types/plugins'

export const usePlugins: UsePlugin = () => {
  const plugins = [
    useControlConcepts(),
    useTargetConcepts(),
    useControlConnections(),
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
    getConceptClasses,
    getConnectionClasses,
    getStyles,
  }
}
