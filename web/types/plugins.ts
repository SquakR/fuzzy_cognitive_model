import { ConceptOutType, ConnectionOutType } from './api'
import cytoscape from 'cytoscape'

export interface Plugin {
  getConceptClasses: (concept: ConceptOutType) => string[]
  getConnectionClasses: (connection: ConnectionOutType) => string[]
  getStyles: () => cytoscape.Stylesheet[]
}

export type UsePlugin = () => Plugin
