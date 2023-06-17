<template>
  <div class="d-flex align-center">
    <span class="mr-2 text-subtitle-1">{{ t('iteration') }}</span>
    <VBtn
      :disabled="iteration === 0"
      class="adjustment-plugin-simulation-iteration-button"
      icon="mdi-chevron-double-left"
      density="compact"
      variant="text"
      @click="iteration = 0"
    />
    <VBtn
      :disabled="iteration === 0"
      class="adjustment-plugin-simulation-iteration-button"
      icon="mdi-chevron-left"
      density="compact"
      variant="text"
      @click="iteration -= 1"
    />
    <span class="ml-1 mr-1 text-subtitle-1">{{ iteration }}</span>
    <VBtn
      :disabled="iteration === maxModelTime"
      class="adjustment-plugin-simulation-iteration-button"
      icon="mdi-chevron-right"
      density="compact"
      variant="text"
      @click="iteration += 1"
    />
    <VBtn
      :disabled="iteration === maxModelTime"
      class="adjustment-plugin-simulation-iteration-button"
      icon="mdi-chevron-double-right"
      density="compact"
      variant="text"
      @click="iteration = maxModelTime"
    />
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

export interface Props {
  modelValue: number
  maxModelTime: number
}
export interface Emits {
  (e: 'update:modelValue', modelValue: number): void
}

const { t } = useI18n()

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const iteration = computed({
  get: () => props.modelValue,
  set: (value) => {
    emit('update:modelValue', Number(value))
  },
})
</script>

<i18n locale="en-US" lang="json">
{
  "iteration": "Iteration:"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "iteration": "Итерация:"
}
</i18n>

<style lang="sass">
.adjustment-plugin-simulation-iteration-button
  border-radius: 4px
</style>
