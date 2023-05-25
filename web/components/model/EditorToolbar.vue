<template>
  <VRow class="mb-1" dense>
    <VCol class="d-flex">
      <VBtnToggle
        v-model="localMode"
        :mandatory="true"
        density="compact"
        color="primary"
      >
        <VBtn class="model-editor-toolbar__button" value="change">{{
          t('change')
        }}</VBtn>
        <VBtn class="model-editor-toolbar__button" value="addConcept">{{
          t('addConcept')
        }}</VBtn>
        <VBtn class="model-editor-toolbar__button" value="addConnection">{{
          t('addConnection')
        }}</VBtn>
      </VBtnToggle>
      <VSpacer />
      <VBtn
        :to="{
          name: 'adjustment-project_id',
          params: { project_id: model.project.id },
        }"
        class="model-editor-toolbar__button"
        color="primary"
        >{{ t('adjustment') }}</VBtn
      >
    </VCol>
  </VRow>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { EditorMode, ModelOutType, Plugins } from '~/types'

export interface Props {
  model: ModelOutType
  mode: EditorMode
  plugins: Plugins
}
export interface Emits {
  (e: 'update:mode', mode: EditorMode): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const { t } = useI18n()

const localMode = computed({
  get: () => props.mode,
  set: (value: EditorMode) => {
    emit('update:mode', value)
  },
})
</script>

<i18n locale="en-US" lang="json">
{
  "change": "Change",
  "addConcept": "Add Concept",
  "addConnection": "Add Connection",
  "adjustment": "Adjustment With Genetic Algorithms"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "change": "Изменить",
  "addConcept": "Добавить концепт",
  "addConnection": "Добавить связь",
  "adjustment": "Структурно-параметрическая настройка"
}
</i18n>

<style lang="sass">
.model-editor-toolbar__button
  text-transform: none
</style>
