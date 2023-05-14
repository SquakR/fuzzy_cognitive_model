import { ConceptOutType, ConnectionOutType } from '~/types'

export const getConceptId = (concept: ConceptOutType | number) => {
  const id = typeof concept === 'number' ? concept : concept.id
  return `concept${id}`
}
export const getConnectionId = (connection: ConnectionOutType) =>
  `connection${connection.id}`
