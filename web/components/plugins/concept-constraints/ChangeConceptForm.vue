<template>
  <BaseForm
    :action-key="CHANGE_CONCEPT_CONSTRAINT_KEY"
    :disabled="!selectedConcept.pluginsData.controlConcepts?.isControl"
    :title="t('title')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    :readonly="readonly"
    width="468"
    flat
  >
    <BaseCheckbox
      :label="t('hasConstraint')"
      :readonly="readonly"
      name="hasConstraint"
    />
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
  CHANGE_CONCEPT_CONSTRAINT_KEY,
  ConceptConstraintsPlugin,
  ConceptOutType,
} from '~/types'

export interface Props {
  selectedConcept: ConceptOutType
  conceptConstraintsPlugin: ConceptConstraintsPlugin
  readonly?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  readonly: false,
})

interface Values {
  hasConstraint: boolean
  minValue: string
  includeMinValue: boolean
  maxValue: string
  includeMaxValue: boolean
}

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

const validationSchema = $yup.object({
  hasConstraint: $yup.boolean().required(),
  minValue: $yup
    .number()
    .required()
    .min(0)
    .when('includeMinValue', {
      is: true,
      then: (schema) => schema.max(props.selectedConcept.value!),
      otherwise: (schema) => schema.lessThan(props.selectedConcept.value!),
    }),
  includeMinValue: $yup.boolean().required(),
  maxValue: $yup
    .number()
    .required()
    .max(1)
    .when('includeMaxValue', {
      is: true,
      then: (schema) => schema.min(props.selectedConcept.value!),
      otherwise: (schema) => schema.moreThan(props.selectedConcept.value!),
    }),
  includeMaxValue: $yup.boolean().required(),
})
const initialValues = computed<Values>(() => {
  const formatter = new Intl.NumberFormat(userStore.locale)
  return {
    hasConstraint:
      props.selectedConcept.pluginsData.conceptConstraints!.hasConstraint,
    minValue: formatter.format(
      props.selectedConcept.pluginsData.conceptConstraints!.minValue
    ),
    includeMinValue:
      props.selectedConcept.pluginsData.conceptConstraints!.includeMinValue,
    maxValue: formatter.format(
      props.selectedConcept.pluginsData.conceptConstraints!.maxValue
    ),
    includeMaxValue:
      props.selectedConcept.pluginsData.conceptConstraints!.includeMaxValue,
  }
})
const onSubmit = async (values: Values) => {
  await props.conceptConstraintsPlugin.changeConceptConstraint(
    props.selectedConcept.id,
    {
      hasConstraint: values.hasConstraint,
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
  "title": "Constraints",
  "hasConstraint": "Has Constraint",
  "minValue": "Min Value",
  "includeMinValue": "Include Min Value",
  "maxValue": "Max Value",
  "includeMaxValue": "Include Max Value",
  "buttonText": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Ограничения",
  "hasConstraint": "Есть ограничение",
  "minValue": "Минимальное значение",
  "includeMinValue": "Включать минимальное значение",
  "maxValue": "Максимальное значение",
  "includeMaxValue": "Включать максимальное значение",
  "buttonText": "Изменить"
}
</i18n>
