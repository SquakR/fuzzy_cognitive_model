<template>
  <VCheckbox
    v-model="value"
    :label="label"
    :error-messages="errors"
    :color="meta.dirty && !errors.length ? 'success' : undefined"
    @blur="handleBlur"
    @click:append-inner="$emit('click:append-inner')"
  />
</template>

<script setup lang="ts">
export interface Props {
  name: string
  label?: string
}
export interface Emits {
  (e: 'click:append-inner'): void
}

const props = withDefaults(defineProps<Props>(), {
  label: undefined,
})
defineEmits<Emits>()

const { value, handleBlur, errors, meta } = useField(
  toRef(props, 'name'),
  undefined,
  { type: 'checkbox' }
)
</script>
