<template>
  <BaseForm
    :action-key="CHANGE_TARGET_CONCEPT_KEY"
    :disabled="selectedConcept.pluginsData.controlConcepts?.isControl"
    :title="t('isTarget')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    width="468"
    flat
  >
    <BaseCheckbox :label="t('isTarget')" name="isTarget" />
    <BaseTextField :label="t('value')" name="value" />
  </BaseForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useUserStore } from '~/store'
import {
  CHANGE_TARGET_CONCEPT_KEY,
  ConceptOutType,
  TargetConceptsPlugin,
} from '~/types'

export interface Props {
  selectedConcept: ConceptOutType
  targetConceptsPlugin: TargetConceptsPlugin
}
const props = defineProps<Props>()

interface Values {
  isTarget: boolean
  value: string
}

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

const validationSchema = $yup.object({
  isTarget: $yup.boolean().required(),
  value: $yup.number().min(0).max(1),
})
const initialValues = computed<Values>(() => {
  const value = String(props.selectedConcept.pluginsData.targetConcepts!.value)
  return {
    isTarget: props.selectedConcept.pluginsData.targetConcepts!.isTarget,
    value: userStore.locale === 'ru-RU' ? value.replace('.', ',') : value,
  }
})
const onSubmit = async (values: Values) => {
  await props.targetConceptsPlugin.changeTargetConcept(
    props.selectedConcept.id,
    {
      isTarget: values.isTarget,
      value: Number(values.value.replace(',', '.')),
    }
  )
}
</script>

<i18n locale="en-US" lang="json">
{
  "isTarget": "Target Concept",
  "value": "Target Value",
  "buttonText": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "isTarget": "Целевой концепт",
  "value": "Целевой значение",
  "buttonText": "Изменить"
}
</i18n>