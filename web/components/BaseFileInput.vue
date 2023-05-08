<template>
  <Field
    v-slot="{ handleChange, handleBlur, meta: { touched }, errors }"
    :name="name"
  >
    <VFileInput
      :variant="variant"
      :label="label"
      :accept="accept"
      :error-messages="errors"
      :color="touched && !errors.length ? 'success' : undefined"
      @change="handleChange"
      @blur="handleBlur"
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
  accept?: string
}

withDefaults(defineProps<Props>(), {
  label: undefined,
  variant: 'underlined',
  accept: undefined,
})
</script>
