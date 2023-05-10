<template>
  <BaseTextField
    :name="name"
    :variant="variant"
    :label="label"
    :type="show ? 'text' : 'password'"
    :counter="true"
    :append-inner-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
    clearable
    @click:append-inner="show = !show"
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
}

withDefaults(defineProps<Props>(), {
  variant: 'underlined',
  label: undefined,
})

const show = ref(false)
</script>
