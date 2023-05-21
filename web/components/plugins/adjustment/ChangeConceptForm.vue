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
    <BaseSelect
      class="plugins-adjustment-change-concept-form__base-select"
      :items="dynamicModelTypeItems"
      :label="t('dynamicModelType')"
      name="dynamicModelType"
      clearable
    >
      <template #selection="{ item }">
        <BaseMathJax :formula="'$$' + item.title + '$$'" />
      </template>
      <template #item="{ item, props }">
        <VListItem v-bind="{ ...props, title: undefined }">
          <BaseMathJax :formula="'$$' + item.title + '$$'" />
        </VListItem>
      </template>
    </BaseSelect>
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

const dynamicModelTypeItems = [
  {
    value: 'delta_delta',
    title: '\\Delta K_j(t+1)=\\sum_{i=1}^Nw_{ij}\\Delta K_i(t)',
  },
  {
    value: 'delta_value',
    title: '\\Delta K_j(t+1)=\\sum_{i=1}^Nw_{ij}K_i(t)',
  },
  {
    value: 'value_delta',
    title: 'K_j(t+1)=\\sum_{i=1}^Nw_{ij}\\Delta K_i(t)',
  },
  { value: 'value_value', title: 'K_j(t+1)=\\sum_{i=1}^Nw_{ij}K_i(t)' },
]

const validationSchema = $yup.object({
  dynamicModelType: $yup
    .string()
    .notRequired()
    .oneOf(dynamicModelTypeItems.map((item) => item.value)),
})
const initialValues = computed<yup.InferType<typeof validationSchema>>(() => ({
  dynamicModelType: props.selectedConcept.pluginsData.adjustment!
    .dynamicModelType
    ? props.selectedConcept.pluginsData.adjustment!.dynamicModelType
    : null,
}))
const onSubmit = async (values: yup.InferType<typeof validationSchema>) => {
  console.log(values)
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

<style lang="sass">
.plugins-adjustment-change-concept-form__base-select
  .MathJax
    font-size: 1.1rem !important
</style>
