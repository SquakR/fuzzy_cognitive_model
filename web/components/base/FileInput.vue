<template>
  <VFileInput
    :variant="variant"
    :label="label"
    :accept="accept"
    :error-messages="errors"
    :color="meta.touched && !errors.length ? 'success' : undefined"
    @change="handleChange"
    @blur="handleBlur"
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
  accept?: string
}

const props = withDefaults(defineProps<Props>(), {
  label: undefined,
  variant: 'underlined',
  accept: undefined,
})

const { handleChange, handleBlur, errors, meta } = useField(
  toRef(props, 'name'),
  undefined
)
</script>
