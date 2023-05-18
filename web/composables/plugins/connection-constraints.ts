import { ModelOutType } from '~/types'
import { UsePlugin } from '~/types/plugins'

export const useConnectionConstraintsPlugin: UsePlugin = (
  model: Ref<ModelOutType>
) => {
  const isInstalled = computed(() =>
    model.value.project.plugins.includes('Connection Constraints')
  )

  const getConceptClasses = () => {
    return []
  }
  const getConnectionClasses = () => {
    return []
  }

  const getStyles = () => {
    return []
  }

  return {
    isInstalled,
    getConceptClasses,
    getConnectionClasses,
    getStyles,
  }
}
