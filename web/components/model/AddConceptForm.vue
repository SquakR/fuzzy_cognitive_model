<template>
  <BaseModalForm
    ref="modelForm"
    v-model="isActive"
    :action-key="CREATE_CONCEPT_KEY"
    :title="t('title')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
  >
    <BaseTextField :label="t('name')" name="name" />
    <BaseTextarea :label="t('description')" name="description" />
    <BaseTextField :label="t('value')" name="value" />
    <BaseTextField :label="t('xPosition')" name="xPosition" />
    <BaseTextField :label="t('yPosition')" name="yPosition" />
  </BaseModalForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import BaseModalForm from '~/components/base/ModalForm.vue'
import { useUserStore } from '~/store'
import { CREATE_CONCEPT_KEY, EditorMode, ModelOutType } from '~/types'

export interface Props {
  model: ModelOutType
  mode: EditorMode
  cy: cytoscape.Core
  createConcept: ReturnType<typeof useModelActions>['createConcept']
  createConceptOnSuccess: ReturnType<
    typeof useModelActions
  >['createConceptOnSuccess']
}

const props = defineProps<Props>()

const modelForm = ref<InstanceType<typeof BaseModalForm> | null>(null)

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

const isActive = ref(false)
const clear = () => {
  isActive.value = false
}

props.createConceptOnSuccess(clear)
watch(isActive, (newValue) => {
  if (!newValue) {
    clear()
  }
})

props.cy.on('click', async (e) => {
  if (props.mode !== 'addConcept') {
    return
  }
  isActive.value = true
  await nextTick()
  modelForm.value?.form?.setValues({
    xPosition: e.position.x.toFixed(0),
    yPosition: e.position.y.toFixed(0),
  })
})

const validationSchema = computed(() => {
  const validationSchema: yup.ObjectShape = {
    name: $yup.string().required().min(1).max(255),
    description: $yup.string(),
    xPosition: $yup.number().required(),
    yPosition: $yup.number().required(),
  }
  if (props.model.project.conceptValueType === 'from_zero_to_one') {
    validationSchema.value = $yup.number().required().min(0).max(1)
  }
  return $yup.object(validationSchema)
})
const initialValues = computed(() => {
  const initialValues: Record<string, string> = {
    name: '',
    description: '',
    xPosition: '0',
    yPosition: '0',
  }
  if (props.model.project.conceptValueType === 'from_zero_to_one') {
    initialValues.value = new Intl.NumberFormat(userStore.locale, {
      minimumFractionDigits: 1,
    }).format(0)
  }
  return initialValues
})
const onSubmit = async (values: Record<string, string>) => {
  props.createConcept(props.model.project.id, {
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
  "title": "Add Concept",
  "buttonText": "Add",
  "name": "Name",
  "description": "Description",
  "value": "Value",
  "xPosition": "Position x",
  "yPosition": "Position y"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Добавление концепта",
  "buttonText": "Добавить",
  "name": "Название",
  "description": "Описание",
  "value": "Значение",
  "xPosition": "Позиция x",
  "yPosition": "Позиция y"
}
</i18n>
