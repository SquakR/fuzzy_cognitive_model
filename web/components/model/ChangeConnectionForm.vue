<template>
  <BaseForm
    :action-key="[CHANGE_CONNECTION_KEY, DELETE_CONNECTION_KEY]"
    :button-text="t('change')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    width="468"
    flat
  >
    <BaseTextarea :label="t('description')" name="description" />
    <BaseTextField :label="t('value')" name="value" />
    <BaseTextField :label="t('source')" name="source" readonly />
    <BaseTextField :label="t('target')" name="target" readonly />
    <template #actions="{ loading, buttonText }">
      <VBtn
        :loading="deleteConnectionPending"
        color="error"
        variant="elevated"
        @click="deleteConnection(selectedConnection.id)"
        >{{ t('delete') }}</VBtn
      >
      <VSpacer />
      <VBtn
        :loading="loading"
        color="primary"
        variant="elevated"
        type="submit"
        >{{ buttonText }}</VBtn
      >
    </template>
  </BaseForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import { useUserStore } from '~/store'
import {
  CHANGE_CONNECTION_KEY,
  ConnectionConstraintsPlugin,
  ConnectionOutType,
  DELETE_CONNECTION_KEY,
  ModelOutType,
} from '~/types'

export interface Props {
  model: ModelOutType
  cy: cytoscape.Core
  selectedConnection: ConnectionOutType
  connectionConstraintPlugin: ConnectionConstraintsPlugin
  changeConnection: ReturnType<typeof useModelActions>['changeConnection']
  deleteConnection: ReturnType<typeof useModelActions>['deleteConnection']
  deleteConnectionPending: boolean
}

const props = defineProps<Props>()

interface Values {
  description: string
  value: string
  source: string
  target: string
}

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

const validationSchema = computed(() => {
  const validationSchema: yup.ObjectShape = {
    description: $yup.string(),
    source: $yup.string().required(),
    target: $yup.string().required(),
  }
  if (props.model.project.connectionValueType === 'symbolic') {
    validationSchema.value = $yup.string().oneOf(['+', '-'])
  } else {
    if (
      props.connectionConstraintPlugin.isInstalled.value &&
      props.selectedConnection.pluginsData.connectionConstraints!.hasConstraint
    ) {
      const constraints =
        props.selectedConnection.pluginsData.connectionConstraints!
      if (constraints.includeMinValue && constraints.includeMaxValue) {
        validationSchema.value = $yup
          .number()
          .required()
          .min(constraints.minValue)
          .max(constraints.maxValue)
      } else if (constraints.includeMinValue) {
        validationSchema.value = $yup
          .number()
          .required()
          .min(constraints.minValue)
          .lessThan(constraints.maxValue)
      } else if (constraints.includeMaxValue) {
        validationSchema.value = $yup
          .number()
          .required()
          .moreThan(constraints.minValue)
          .max(constraints.maxValue)
      } else {
        validationSchema.value = $yup
          .number()
          .required()
          .moreThan(constraints.minValue)
          .lessThan(constraints.maxValue)
      }
    } else {
      validationSchema.value = $yup.number().required().min(-1).max(1)
    }
  }
  return $yup.object(validationSchema)
})
const initialValues = computed<Values>(() => {
  const initialValues: Values = {
    description: props.selectedConnection.description,
    value: '',
    source: props.model.concepts.find(
      (concept) => concept.id === props.selectedConnection.sourceId
    )!.name,
    target: props.model.concepts.find(
      (concepts) => concepts.id === props.selectedConnection.targetId
    )!.name,
  }
  if (props.model.project.connectionValueType === 'symbolic') {
    initialValues.value = props.selectedConnection.value === 0 ? '-' : '+'
  } else {
    initialValues.value =
      userStore.locale === 'ru-RU'
        ? String(props.selectedConnection.value).replace('.', ',')
        : String(props.selectedConnection.value)
  }
  return initialValues
})
const onSubmit = async (values: Values) => {
  let value
  if (props.model.project.connectionValueType === 'symbolic') {
    value = values.value === '+' ? 1 : -1
  } else {
    value = Number(values.value.replace(',', '.'))
  }
  props.changeConnection(props.selectedConnection.id, {
    description: values.description,
    value,
  })
}
</script>

<i18n locale="en-US" lang="json">
{
  "description": "Description",
  "value": "Value",
  "source": "Source concept",
  "target": "Target concept",
  "delete": "Delete",
  "change": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "description": "Описание",
  "value": "Значение",
  "source": "Начальный концепт",
  "target": "Конечный концепт",
  "delete": "Удалить",
  "change": "Изменить"
}
</i18n>
