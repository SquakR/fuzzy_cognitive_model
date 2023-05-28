<template>
  <VCheckbox
    v-model="value"
    :label="label"
    :error-messages="errors"
    :readonly="readonly"
    color="primary"
    @blur="handleBlur"
    @click:append-inner="$emit('click:append-inner')"
  />
</template>

<script setup lang="ts">
export interface Props {
  name: string
  label?: string
  readonly?: boolean
}
export interface Emits {
  (e: 'click:append-inner'): void
}

const props = withDefaults(defineProps<Props>(), {
  label: undefined,
  readonly: undefined,
})
defineEmits<Emits>()

const { value, handleBlur, errors } = useField(
  toRef(props, 'name'),
  undefined,
  { type: 'checkbox' }
)
</script>
