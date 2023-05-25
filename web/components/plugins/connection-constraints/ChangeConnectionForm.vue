<template>
  <BaseForm
    :action-key="CHANGE_CONNECTION_CONSTRAINT_KEY"
    :disabled="!selectedConnection.pluginsData.controlConnections?.isControl"
    :title="t('title')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    width="468"
    flat
  >
    <BaseCheckbox :label="t('hasConstraint')" name="hasConstraint" />
    <BaseTextField :label="t('minValue')" name="minValue" />
    <BaseCheckbox :label="t('includeMinValue')" name="includeMinValue" />
    <BaseTextField :label="t('maxValue')" name="maxValue" />
    <BaseCheckbox :label="t('includeMaxValue')" name="includeMaxValue" />
  </BaseForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useUserStore } from '~/store'
import {
  CHANGE_CONNECTION_CONSTRAINT_KEY,
  ConnectionConstraintsPlugin,
  ConnectionOutType,
} from '~/types'

export interface Props {
  selectedConnection: ConnectionOutType
  connectionConstraintsPlugin: ConnectionConstraintsPlugin
}

const props = defineProps<Props>()

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
      then: (schema) => schema.max(props.selectedConnection.value!),
      otherwise: (schema) => schema.lessThan(props.selectedConnection.value!),
    }),
  includeMinValue: $yup.boolean().required(),
  maxValue: $yup
    .number()
    .required()
    .max(1)
    .when('includeMaxValue', {
      is: true,
      then: (schema) => schema.min(props.selectedConnection.value!),
      otherwise: (schema) => schema.moreThan(props.selectedConnection.value!),
    }),
  includeMaxValue: $yup.boolean().required(),
})
const initialValues = computed<Values>(() => {
  const minValue = String(
    props.selectedConnection.pluginsData.connectionConstraints!.minValue
  )
  const maxValue = String(
    props.selectedConnection.pluginsData.connectionConstraints!.maxValue
  )
  return {
    hasConstraint:
      props.selectedConnection.pluginsData.connectionConstraints!.hasConstraint,
    minValue:
      userStore.locale === 'ru-RU' ? minValue.replace('.', ',') : minValue,
    includeMinValue:
      props.selectedConnection.pluginsData.connectionConstraints!
        .includeMinValue,
    maxValue:
      userStore.locale === 'ru-RU' ? maxValue.replace('.', ',') : maxValue,
    includeMaxValue:
      props.selectedConnection.pluginsData.connectionConstraints!
        .includeMaxValue,
  }
})
const onSubmit = async (values: Values) => {
  await props.connectionConstraintsPlugin.changeConnectionConstraint(
    props.selectedConnection.id,
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
