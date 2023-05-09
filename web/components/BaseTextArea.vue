<template>
  <Field v-slot="{ field, meta: { dirty }, errors }" :name="name">
    <VTextarea
      v-bind="field"
      :variant="variant"
      :label="label"
      :clearable="clearable"
      :error-messages="errors"
      :counter="counter"
      :append-inner-icon="appendInnerIcon"
      :color="dirty && !errors.length ? 'success' : undefined"
      @click:append-inner="$emit('click:append-inner')"
    />
  </Field>
</template>

<script setup lang="ts">
import { Field } from 'vee-validate'

export interface Props {
  name: string
  variant?:
    | 'outlined'
    | 'plain'
    | 'underlined'
    | 'filled'
    | 'solo'
    | 'solo-inverted'
    | 'solo-filled'
  label?: string
  clearable?: boolean
  counter?: string | number | true
  appendInnerIcon?: string
}
export interface Emits {
  (e: 'click:append-inner'): void
}

withDefaults(defineProps<Props>(), {
  variant: 'underlined',
  label: undefined,
  clearable: false,
  counter: undefined,
  appendInnerIcon: undefined,
})
defineEmits<Emits>()
</script>
