<template>
  <component
    v-bind="$attrs"
    :is="tag"
    :class="computedClass"
    :type="type"
    :disabled="disabled"
    ><slot></slot
  ></component>
</template>

<script lang="ts">
export default {
  inheritAttrs: false,
}
</script>

<script setup lang="ts">
export interface Props {
  tag?: string
  color?: string
  type?: string
  disabled?: boolean
  loading?: boolean
  class?: string | Record<string, string> | string[] | null
}
const props = withDefaults(defineProps<Props>(), {
  tag: 'button',
  color: 'primary',
  type: 'button',
  disabled: false,
  loading: false,
  class: null,
})

const computedClass = computed(() => {
  const result: (string | Record<string, string>)[] = [
    'button',
    `is-${props.color}`,
  ]
  if (props.class) {
    if (Array.isArray(props.class)) {
      result.push(...props.class)
    } else {
      result.push(props.class)
    }
  }
  if (props.loading) {
    result.push('is-loading')
  }
  return result
})
</script>
