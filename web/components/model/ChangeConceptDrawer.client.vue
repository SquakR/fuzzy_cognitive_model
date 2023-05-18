<template>
  <Teleport to=".editor-layout__right-menu">
    <VExpandXTransition>
      <VCard v-show="isActive" class="model-change-concept-form" width="500">
        <VTabs v-model="tab" bg-color="teal-lighten-1">
          <VTab value="concept">{{ t('concept') }}</VTab>
          <VTab v-if="plugins.adjustment.isInstalled" value="adjustment">{{
            t('adjustment')
          }}</VTab>
        </VTabs>
        <VCardText>
          <VWindow v-model="tab">
            <VWindowItem value="concept"
              ><ModelChangeConceptForm
                v-if="selectedConcept"
                :model="model"
                :cy="cy"
                :selected-concept="selectedConcept"
                :change-concept="changeConcept"
            /></VWindowItem>
            <VWindowItem
              v-if="plugins.adjustment.isInstalled"
              value="adjustment"
              >Adjustment</VWindowItem
            >
          </VWindow>
        </VCardText>
      </VCard>
    </VExpandXTransition>
  </Teleport>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ConceptOutType, EditorMode, ModelOutType, Plugins } from '~/types'

export interface Props {
  model: ModelOutType
  plugins: Plugins
  mode: EditorMode
  cy: cytoscape.Core
  changeConcept: ReturnType<typeof useModelActions>['changeConcept']
  changeConceptOnSuccess: ReturnType<
    typeof useModelActions
  >['changeConceptOnSuccess']
}
const props = defineProps<Props>()

const { t } = useI18n()

const selectedConcept = ref<ConceptOutType | null>(null)
const isActive = computed(() => Boolean(selectedConcept.value))

const tab = ref(null)

props.changeConceptOnSuccess(() => {
  props.cy.$('node:selected').unselect()
})

props.cy.on('select', 'node', (e) => {
  selectedConcept.value = props.model.concepts.find(
    (concept) => concept.id === e.target.data().conceptId
  )!
})
props.cy.on('unselect', 'node', () => {
  selectedConcept.value = null
})
props.cy.on('remove', 'node', () => {
  selectedConcept.value = null
})
</script>

<style lang="sass">
.model-change-concept-form
  pointer-events: auto
  position: absolute
  height: 100%
  right: 0
</style>

<i18n locale="en-US" lang="json">
{
  "concept": "Concept",
  "adjustment": "Adjustment"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "concept": "Концепт",
  "adjustment": "Настройка"
}
</i18n>
