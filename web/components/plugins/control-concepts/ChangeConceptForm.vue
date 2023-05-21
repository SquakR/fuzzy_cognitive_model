<template>
  <BaseForm
    :action-key="SET_IS_CONTROL_KEY"
    :disabled="selectedConcept.pluginsData.targetConcepts?.isTarget"
    :title="t('isControl')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    width="468"
    flat
  >
    <BaseCheckbox :label="t('isControl')" name="isControl" />
  </BaseForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import {
  ConceptOutType,
  ControlConceptsPlugin,
  SET_IS_CONTROL_KEY,
} from '~/types'

export interface Props {
  selectedConcept: ConceptOutType
  controlConceptsPlugin: ControlConceptsPlugin
}
const props = defineProps<Props>()

const { $yup } = useNuxtApp()
const { t } = useI18n()

const validationSchema = $yup.object({
  isControl: $yup.boolean().required(),
})
const initialValues: yup.InferType<typeof validationSchema> = {
  isControl: props.selectedConcept.pluginsData.controlConcepts!.isControl,
}
const onSubmit = async (values: yup.InferType<typeof validationSchema>) => {
  await props.controlConceptsPlugin.setIsControl(
    props.selectedConcept.id,
    values.isControl
  )
}
</script>

<i18n locale="en-US" lang="json">
{
  "isControl": "Control Concept",
  "buttonText": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "isControl": "Управляющий концепт",
  "buttonText": "Изменить"
}
</i18n>
