<template>
  <div class="field base-text-field__field">
    <label class="label">{{ label }}</label>
    <div class="control">
      <Field
        v-slot="{ field, meta: { touched }, errors }"
        v-model="value"
        :name="name"
      >
        <input
          v-bind="field"
          :placeholder="placeholder"
          class="input"
          :class="{
            'is-danger': errors.length,
            'is-success': touched && !errors.length,
          }"
        />
      </Field>
    </div>
    <ErrorMessage :name="name" class="help is-danger" />
  </div>
</template>

<script setup lang="ts">
import { Field, ErrorMessage } from 'vee-validate'

export interface Props {
  modelValue: string
  name: string
  label?: string
  placeholder?: string
}
export interface Emits {
  (e: 'update:modelValue', value: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const value = computed({
  get() {
    return props.modelValue
  },
  set(value) {
    emit('update:modelValue', value)
  },
})
</script>

<style lang="sass">
.base-text-field__field
  height: 94px
</style>
