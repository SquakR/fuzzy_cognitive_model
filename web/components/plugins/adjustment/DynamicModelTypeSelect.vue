<template>
  <BaseSelect
    class="plugins-adjustment-dynamic-model-type-select"
    :items="dynamicModelTypeItems"
    :label="t('dynamicModelType')"
    :readonly="readonly"
    name="dynamicModelType"
    clearable
  >
    <template #selection="{ item }">
      <BaseMathJax :formula="item.title" />
    </template>
    <template #item="{ item, props }">
      <VListItem v-bind="{ ...props, title: undefined }">
        <BaseMathJax :formula="item.title" />
      </VListItem>
    </template>
  </BaseSelect>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

export interface Props {
  readonly?: boolean
}

withDefaults(defineProps<Props>(), {
  readonly: false,
})

const { t } = useI18n()

const dynamicModelTypeItems = [
  {
    value: 'delta_delta',
    title: '\\[\\Delta K_j(t+1)=\\sum_{i=1}^Nw_{ij}\\Delta K_i(t)\\]',
  },
  {
    value: 'delta_value',
    title: '\\[\\Delta K_j(t+1)=\\sum_{i=1}^Nw_{ij}K_i(t)\\]',
  },
  {
    value: 'value_delta',
    title: '\\[K_j(t+1)=\\sum_{i=1}^Nw_{ij}\\Delta K_i(t)\\]',
  },
  { value: 'value_value', title: '\\[K_j(t+1)=\\sum_{i=1}^Nw_{ij}K_i(t)\\]' },
]
</script>

<i18n locale="en-US" lang="json">
{
  "dynamicModelType": "Dynamic Model"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "dynamicModelType": "Модель динамики"
}
</i18n>

<style lang="sass">
.plugins-adjustment-dynamic-model-type-select
  .MathJax
    font-size: 1.1rem !important
</style>
