<template>
  <VSelect
    v-model="value"
    :items="items"
    :variant="variant"
    :label="label"
    :readonly="readonly"
    :clearable="clearable"
    :error-messages="errors"
    :color="meta.dirty && !errors.length ? 'success' : undefined"
    @blur="handleBlur"
  >
    <template v-if="$slots.selection" #selection="data">
      <slot name="selection" v-bind="data" />
    </template>
    <template v-if="$slots.item" #item="data">
      <slot name="item" v-bind="data" />
    </template>
  </VSelect>
</template>

<script setup lang="ts">
export interface Props {
  name: string
  items: any[]
  variant?:
    | 'outlined'
    | 'plain'
    | 'underlined'
    | 'filled'
    | 'solo'
    | 'solo-inverted'
    | 'solo-filled'
  label?: string
  readonly?: boolean
  clearable?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'underlined',
  label: undefined,
  readonly: undefined,
  clearable: undefined,
})

const { value, handleBlur, errors, meta } = useField(toRef(props, 'name'))
</script>
