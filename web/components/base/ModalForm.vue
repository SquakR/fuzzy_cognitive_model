<template>
  <VDialog v-model="isActive" :width="width" persistent>
    <template #activator="{ props }">
      <slot name="activator" :props="props" />
    </template>
    <BaseForm
      ref="baseForm"
      :action-key="actionKey"
      :disabled="disabled"
      :title="title"
      :subtitle="subtitle"
      :button-text="buttonText"
      :validation-schema="validationSchema"
      :initial-values="initialValues"
      :on-submit="onSubmit"
      :width="width"
      :success-message="successMessage"
    >
      <template #title="{ title }">
        <div class="d-flex">
          <slot name="title" :title="title">{{ title }}</slot>
          <VSpacer />
          <VBtn icon="mdi-close" variant="flat" @click="close" />
        </div>
      </template>
      <slot />
      <template #actions="{ disabled, loading, buttonText }">
        <slot name="actions" :loading="loading" :button-text="buttonText">
          <VSpacer />
          <VBtn
            :disabled="disabled"
            :loading="loading"
            color="primary"
            variant="elevated"
            type="submit"
            >{{ buttonText }}</VBtn
          >
        </slot>
      </template>
    </BaseForm>
  </VDialog>
</template>

<script setup lang="ts">
import { SubmissionHandler } from 'vee-validate'
import BaseForm from '~/components/base/Form.vue'

export interface Props {
  modelValue: boolean
  actionKey: string | string[]
  title: string
  buttonText: string
  validationSchema: object
  initialValues: Record<string, any>
  onSubmit: SubmissionHandler<any>
  disabled?: boolean
  subtitle?: string
  width?: string | number
  successMessage?: string
}
export interface Emits {
  (e: 'update:modelValue', modelValue: boolean): void
  (e: 'close'): void
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  subtitle: undefined,
  width: 500,
  successMessage: undefined,
})
const emit = defineEmits<Emits>()

const baseForm = ref<InstanceType<typeof BaseForm> | null>(null)
const form = computed(() => (baseForm.value ? baseForm.value.form : null))
defineExpose({ form })

const isActive = computed({
  get: () => props.modelValue,
  set: (value) => {
    emit('update:modelValue', value)
  },
})

const close = () => {
  isActive.value = false
  emit('close')
}
</script>
