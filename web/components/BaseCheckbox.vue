<template>
  <Field
    v-slot="{ handleChange, handleBlur, meta: { dirty }, errors }"
    :name="name"
  >
    <VCheckbox
      :label="label"
      :error-messages="errors"
      :color="dirty && !errors.length ? 'success' : undefined"
      @change="handleChange"
      @blur="handleBlur"
      @click:append-inner="$emit('click:append-inner')"
    />
  </Field>
</template>

<script setup lang="ts">
import { Field } from 'vee-validate'

export interface Props {
  name: string
  label?: string
}
export interface Emits {
  (e: 'click:append-inner'): void
}

withDefaults(defineProps<Props>(), {
  label: undefined,
})
defineEmits<Emits>()
</script>
