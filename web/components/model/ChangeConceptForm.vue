<template>
  <BaseForm
    ref="form"
    :action-key="[CHANGE_CONCEPT_KEY, DELETE_CONCEPT_KEY]"
    :button-text="t('change')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    :readonly="readonly"
    width="468"
    flat
  >
    <BaseTextField :label="t('name')" :readonly="readonly" name="name" />
    <BaseTextarea
      :label="t('description')"
      :readonly="readonly"
      name="description"
    />
    <BaseTextField :label="t('value')" :readonly="readonly" name="value" />
    <BaseTextField
      :label="t('xPosition')"
      :readonly="readonly"
      name="xPosition"
    />
    <BaseTextField
      :label="t('yPosition')"
      :readonly="readonly"
      name="yPosition"
    />
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
  readonly?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  readonly: false,
})

const form = ref<InstanceType<typeof BaseForm> | null>(null)

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

const positionFormatter = new Intl.NumberFormat(userStore.locale, {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
})

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
      xPosition: positionFormatter.format(position.x),
      yPosition: positionFormatter.format(position.y),
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
    xPosition: positionFormatter.format(props.selectedConcept.xPosition),
    yPosition: positionFormatter.format(props.selectedConcept.yPosition),
  }
  if (props.model.project.conceptValueType === 'from_zero_to_one') {
    initialValues.value = new Intl.NumberFormat(userStore.locale, {
      maximumFractionDigits: 5,
    }).format(props.selectedConcept.value!)
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
