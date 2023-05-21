<template>
  <BaseModalForm
    v-model="isActive"
    :action-key="CREATE_CONNECTION_KEY"
    :title="t('title')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
  >
    <BaseTextarea :label="t('description')" name="description" />
    <BaseTextField :label="t('value')" name="value" />
    <BaseTextField :label="t('source')" name="source" readonly />
    <BaseTextField :label="t('target')" name="target" readonly />
  </BaseModalForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import { useUserStore } from '~/store'
import {
  CREATE_CONNECTION_KEY,
  ConceptOutType,
  EditorMode,
  ModelOutType,
} from '~/types'

export interface Props {
  model: ModelOutType
  mode: EditorMode
  cy: cytoscape.Core
  createConnection: ReturnType<typeof useModelActions>['createConnection']
  createConnectionOnSuccess: ReturnType<
    typeof useModelActions
  >['createConnectionOnSuccess']
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

const isActive = ref(false)

const source = ref<ConceptOutType | null>(null)
const target = ref<ConceptOutType | null>(null)
const updateClasses = (
  newValue: ConceptOutType | null,
  oldValue: ConceptOutType | null,
  className: string
) => {
  if (oldValue) {
    props.cy.$(`#${getConceptId(oldValue)}`).removeClass(className)
  }
  if (newValue) {
    props.cy.$(`#${getConceptId(newValue)}`).addClass(className)
  }
}
watch(source, (newValue, oldValue) =>
  updateClasses(newValue, oldValue, 'add-connection-source')
)
watch(target, (newValue, oldValue) =>
  updateClasses(newValue, oldValue, 'add-connection-target')
)
const clear = () => {
  isActive.value = false
  source.value = null
  target.value = null
}
defineExpose({ clear })

props.createConnectionOnSuccess(clear)
watch(isActive, (newValue) => {
  if (!newValue) {
    clear()
  }
})
onKeyStroke('Escape', () => {
  if (!isActive.value) {
    clear()
  }
})

props.cy.on('click', (e) => {
  if (props.mode !== 'addConnection') {
    return
  }
  if (e.target === props.cy || !e.target.isNode()) {
    source.value = null
    target.value = null
    return
  }
  const concept = props.model.concepts.find(
    (concept) => concept.id === e.target.data().conceptId
  )!
  if (source.value) {
    if (
      source.value.id !== concept.id &&
      !props.model.connections.find(
        (connection) =>
          connection.sourceId === source.value!.id &&
          connection.targetId === concept.id
      )
    ) {
      target.value = concept
      isActive.value = true
    }
  } else {
    source.value = concept
  }
})

const validationSchema = computed(() => {
  const validationSchema: yup.ObjectShape = {
    description: $yup.string(),
    source: $yup.string().required(),
    target: $yup.string().required(),
  }
  if (props.model.project.connectionValueType === 'symbolic') {
    validationSchema.value = $yup.string().oneOf(['+', '-'])
  } else {
    validationSchema.value = $yup.number().required().min(-1).max(1)
  }
  return $yup.object(validationSchema)
})
const initialValues = computed<Values>(() => {
  const initialValues: Values = {
    description: '',
    value: '',
    source: source.value?.name || '',
    target: target.value?.name || '',
  }
  if (props.model.project.connectionValueType === 'symbolic') {
    initialValues.value = '+'
  } else {
    initialValues.value = userStore.locale === 'ru-RU' ? '0,0' : '0.0'
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
  props.createConnection(props.model.project.id, {
    description: values.description,
    value,
    sourceId: source.value!.id,
    targetId: target.value!.id,
  })
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Add Connection",
  "buttonText": "Add",
  "description": "Description",
  "value": "Value",
  "source": "Source concept",
  "target": "Target concept"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Добавление связи",
  "buttonText": "Добавить",
  "description": "Описание",
  "value": "Значение",
  "source": "Начальный концепт",
  "target": "Конечный концепт"
}
</i18n>
