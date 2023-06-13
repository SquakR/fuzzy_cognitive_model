<template>
  <ModelDrawer
    v-if="cy"
    :model="model"
    :plugins="plugins"
    :cy="cy"
    :change-concept="changeConcept"
    :delete-concept="deleteConcept"
    :delete-concept-pending="deleteConceptPending"
    :change-connection="changeConnection"
    :delete-connection="deleteConnection"
    :delete-connection-pending="deleteConnectionPending"
    readonly
  />
  <div
    ref="container"
    class="plugins-adjustment-editor__cytoscape-container model__cytoscape-container"
  ></div>
</template>

<script setup lang="ts">
import { useUserStore } from '~/store'
import { EditorMode, ModelOutType } from '~/types'

export interface Props {
  model: ModelOutType
}

const props = defineProps<Props>()

const userStore = useUserStore()

const { container, cy, plugins } = useModel(toRef(props, 'model'))

const mode = ref<EditorMode>('change')

watch(
  () => props.model.concepts,
  (newValue) => {
    for (const concept of newValue) {
      setConceptDataWithPosition(
        cy.value!,
        props.model,
        concept,
        userStore.locale
      )
    }
  },
  { deep: true }
)

const {
  changeConcept,
  deleteConcept,
  deleteConceptPending,
  changeConnection,
  deleteConnection,
  deleteConnectionPending,
} = useModelActions(toRef(props, 'model'), mode, cy, plugins)
</script>

<style lang="sass">
.plugins-adjustment-editor__cytoscape-container
  height: 100%
</style>
