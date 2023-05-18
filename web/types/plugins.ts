import { ConceptOutType, ConnectionOutType, ModelOutType } from './api'
import cytoscape from 'cytoscape'

export interface Plugins {
  getConceptClasses: (concept: ConceptOutType) => string[]
  getConnectionClasses: (connection: ConnectionOutType) => string[]
  getStyles: () => cytoscape.Stylesheet[]
  controlConcepts: Plugin
  targetConcepts: Plugin
  controlConnections: Plugin
  conceptConstraints: Plugin
  connectionConstraints: Plugin
  adjustment: Plugin
}

export type UsePlugins = (model: Ref<ModelOutType>) => Plugins

export interface Plugin {
  isInstalled: ComputedRef<boolean>
  getConceptClasses: (concept: ConceptOutType) => string[]
  getConnectionClasses: (connection: ConnectionOutType) => string[]
  getStyles: () => cytoscape.Stylesheet[]
}

export type UsePlugin = (model: Ref<ModelOutType>) => Plugin
