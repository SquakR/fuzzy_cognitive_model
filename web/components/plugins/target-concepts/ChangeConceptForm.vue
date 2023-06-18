<template>
  <BaseForm
    :action-key="CHANGE_TARGET_CONCEPT_KEY"
    :disabled="selectedConcept.pluginsData.controlConcepts?.isControl"
    :title="t('isTarget')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    :readonly="readonly"
    width="468"
    flat
  >
    <BaseCheckbox :label="t('isTarget')" :readonly="readonly" name="isTarget" />
    <BaseTextField
      :label="t('minValue')"
      :readonly="readonly"
      name="minValue"
    />
    <BaseCheckbox
      :label="t('includeMinValue')"
      :readonly="readonly"
      name="includeMinValue"
    />
    <BaseTextField
      :label="t('maxValue')"
      :readonly="readonly"
      name="maxValue"
    />
    <BaseCheckbox
      :label="t('includeMaxValue')"
      :readonly="readonly"
      name="includeMaxValue"
    />
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
  readonly?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  readonly: false,
})

interface Values {
  isTarget: boolean
  minValue: string
  includeMinValue: boolean
  maxValue: string
  includeMaxValue: boolean
}

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

const validationSchema = $yup.object({
  isTarget: $yup.boolean().required(),
  value: $yup.number().min(0).max(1),
})
const initialValues = computed<Values>(() => {
  const formatter = new Intl.NumberFormat(userStore.locale)
  return {
    isTarget: props.selectedConcept.pluginsData.targetConcepts!.isTarget,
    minValue: formatter.format(
      props.selectedConcept.pluginsData.targetConcepts!.minValue!
    ),
    includeMinValue:
      props.selectedConcept.pluginsData.targetConcepts!.includeMinValue!,
    maxValue: formatter.format(
      props.selectedConcept.pluginsData.targetConcepts!.maxValue!
    ),
    includeMaxValue:
      props.selectedConcept.pluginsData.targetConcepts!.includeMaxValue!,
  }
})
const onSubmit = async (values: Values) => {
  await props.targetConceptsPlugin.changeTargetConcept(
    props.selectedConcept.id,
    {
      isTarget: values.isTarget,
      minValue: Number(values.minValue.replace(',', '.')),
      includeMinValue: values.includeMinValue,
      maxValue: Number(values.maxValue.replace(',', '.')),
      includeMaxValue: values.includeMaxValue,
    }
  )
}
</script>

<i18n locale="en-US" lang="json">
{
  "isTarget": "Target Concept",
  "minValue": "Min Value",
  "includeMinValue": "Include Min Value",
  "maxValue": "Max Value",
  "includeMaxValue": "Include Max Value",
  "buttonText": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "isTarget": "Целевой концепт",
  "minValue": "Минимальное значение",
  "includeMinValue": "Включать минимальное значение",
  "maxValue": "Максимальное значение",
  "includeMaxValue": "Включать максимальное значение",
  "buttonText": "Изменить"
}
</i18n>
