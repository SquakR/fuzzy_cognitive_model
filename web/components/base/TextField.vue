<template>
  <VTextField
    v-model="value"
    :variant="variant"
    :label="label"
    :type="type"
    :readonly="readonly"
    :clearable="clearable"
    :error-messages="errors"
    :counter="counter"
    :append-inner-icon="appendInnerIcon"
    :color="meta.dirty && !errors.length ? 'success' : undefined"
    @blur="handleBlur"
    @click:append-inner="$emit('click:append-inner')"
  />
</template>

<script setup lang="ts">
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
  type?: string
  readonly?: boolean
  clearable?: boolean
  counter?: string | number | true
  appendInnerIcon?: string
}
export interface Emits {
  (e: 'click:append-inner'): void
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'underlined',
  label: undefined,
  type: 'text',
  readonly: undefined,
  clearable: false,
  counter: undefined,
  appendInnerIcon: undefined,
})
defineEmits<Emits>()

const { value, handleBlur, errors, meta } = useField(toRef(props, 'name'))
</script>
