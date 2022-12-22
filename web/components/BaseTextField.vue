<template>
  <div class="field">
    <label class="label">{{ label }}</label>
    <div class="control">
      <input
        v-model="value"
        class="input"
        type="text"
        :placeholder="placeholder"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
export interface Props {
  label?: string
  placeholder?: string
  modelValue: string
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
