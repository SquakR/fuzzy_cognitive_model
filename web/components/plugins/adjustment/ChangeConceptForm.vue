<template>
  <BaseForm
    :action-key="CHANGE_DYNAMIC_MODEL_TYPE_KEY"
    :title="t('dynamicModelType')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    width="468"
    flat
  >
    <PluginsAdjustmentDynamicModelTypeSelect />
  </BaseForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import {
  AdjustmentPlugin,
  CHANGE_DYNAMIC_MODEL_TYPE_KEY,
  ConceptOutType,
  DynamicModelType,
} from '~/types'

export interface Props {
  selectedConcept: ConceptOutType
  adjustmentPlugin: AdjustmentPlugin
}

const props = defineProps<Props>()

const { $yup } = useNuxtApp()
const { t } = useI18n()

const validationSchema = $yup.object({
  dynamicModelType: $yup
    .string()
    .notRequired()
    .oneOf(['delta_delta', 'delta_value', 'value_delta', 'value_value']),
})
const initialValues = computed<yup.InferType<typeof validationSchema>>(() => ({
  dynamicModelType: props.selectedConcept.pluginsData.adjustment!
    .dynamicModelType
    ? props.selectedConcept.pluginsData.adjustment!.dynamicModelType
    : null,
}))
const onSubmit = async (values: yup.InferType<typeof validationSchema>) => {
  await props.adjustmentPlugin.changeDynamicModelType(
    props.selectedConcept.id,
    values.dynamicModelType
      ? (values.dynamicModelType as DynamicModelType)
      : null
  )
}
</script>

<i18n locale="en-US" lang="json">
{
  "dynamicModelType": "Dynamic Model",
  "buttonText": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "dynamicModelType": "Модель динамики",
  "buttonText": "Изменить"
}
</i18n>
