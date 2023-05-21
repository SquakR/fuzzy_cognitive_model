<template>
  <BaseForm
    ref="form"
    :action-key="[CHANGE_CONCEPT_KEY, DELETE_CONCEPT_KEY]"
    :button-text="t('change')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    width="468"
    flat
  >
    <BaseTextField :label="t('name')" name="name" />
    <BaseTextarea :label="t('description')" name="description" />
    <BaseTextField :label="t('value')" name="value" />
    <BaseTextField :label="t('xPosition')" name="xPosition" />
    <BaseTextField :label="t('yPosition')" name="yPosition" />
    <template #actions="{ loading, buttonText }">
      <VBtn
        :loading="deleteConceptPending"
        color="error"
        variant="elevated"
        @click="deleteConcept(selectedConcept.id)"
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
import BaseForm from '~/components/base/Form.vue'
import { useUserStore } from '~/store'
import {
  CHANGE_CONCEPT_KEY,
  ConceptConstraintsPlugin,
  ConceptOutType,
  DELETE_CONCEPT_KEY,
  ModelOutType,
} from '~/types'

export interface Props {
  model: ModelOutType
  cy: cytoscape.Core
  selectedConcept: ConceptOutType
  conceptConstraintsPlugin: ConceptConstraintsPlugin
  changeConcept: ReturnType<typeof useModelActions>['changeConcept']
  deleteConcept: ReturnType<typeof useModelActions>['deleteConcept']
  deleteConceptPending: boolean
}
const props = defineProps<Props>()

const form = ref<InstanceType<typeof BaseForm> | null>(null)

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

onMounted(() => {
  props.cy.on('drag', 'node', onDrag)
})
onUnmounted(() => {
  props.cy.removeListener('drag', 'node', onDrag)
})
const onDrag = (e: cytoscape.EventObject) => {
  if (props.selectedConcept.id === e.target.data().conceptId) {
    const position = e.target.position()
    form.value?.form?.setValues({
      xPosition:
        userStore.locale === 'ru-RU'
          ? position.x.toFixed(2).replace('.', ',')
          : position.x.toFixed(2),
      yPosition:
        userStore.locale === 'ru-RU'
          ? position.y.toFixed(2).replace('.', ',')
          : position.y.toFixed(2),
    })
  }
}

const validationSchema = computed(() => {
  const validationSchema: yup.ObjectShape = {
    name: $yup.string().required().min(1).max(255),
    description: $yup.string(),
    xPosition: $yup.number().required(),
    yPosition: $yup.number().required(),
  }
  if (props.model.project.conceptValueType === 'from_zero_to_one') {
    if (
      props.conceptConstraintsPlugin.isInstalled.value &&
      props.selectedConcept.pluginsData.conceptConstraints!.hasConstraint
    ) {
      const constraints = props.selectedConcept.pluginsData.conceptConstraints!
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
      validationSchema.value = $yup.number().required().min(0).max(1)
    }
  }
  return $yup.object(validationSchema)
})
const initialValues = computed(() => {
  const initialValues: Record<string, string> = {
    name: props.selectedConcept.name,
    description: props.selectedConcept.description,
    xPosition:
      userStore.locale === 'ru-RU'
        ? props.selectedConcept.xPosition.toFixed(2).replace('.', ',')
        : props.selectedConcept.xPosition.toFixed(2),
    yPosition:
      userStore.locale === 'ru-RU'
        ? props.selectedConcept.yPosition.toFixed(2).replace('.', ',')
        : props.selectedConcept.yPosition.toFixed(2),
  }
  if (props.model.project.conceptValueType === 'from_zero_to_one') {
    const value = String(props.selectedConcept.value)
    initialValues.value =
      userStore.locale === 'ru-RU' ? value.replace('.', ',') : value
  }
  return initialValues
})
const onSubmit = async (values: Record<string, string>) => {
  await props.changeConcept(props.selectedConcept.id, {
    name: values.name,
    description: values.description,
    value: values.value ? Number(values.value.replace(',', '.')) : null,
    xPosition: Number(values.xPosition.replace(',', '.')),
    yPosition: Number(values.yPosition.replace(',', '.')),
  })
}
</script>

<i18n locale="en-US" lang="json">
{
  "name": "Name",
  "description": "Description",
  "value": "Value",
  "xPosition": "Position x",
  "yPosition": "Position y",
  "delete": "Delete",
  "change": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "name": "Название",
  "description": "Описание",
  "value": "Значение",
  "xPosition": "Позиция x",
  "yPosition": "Позиция y",
  "delete": "Удалить",
  "change": "Изменить"
}
</i18n>
