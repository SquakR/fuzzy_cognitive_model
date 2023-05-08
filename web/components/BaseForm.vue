<template>
  <Form
    v-slot="{ isSubmitting }"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="handleSubmit"
  >
    <VCard :width="computedWidth">
      <VCardTitle>
        <slot name="title">{{ title }}</slot>
      </VCardTitle>
      <VCardText>
        <VAlert
          v-if="message"
          :color="message.type"
          class="mb-2"
          closable
          @click:close="message = null"
          >{{ message.message }}</VAlert
        >
        <slot />
      </VCardText>
      <VCardActions>
        <VSpacer />
        <VBtn
          color="primary"
          variant="elevated"
          type="submit"
          :loading="isSubmitting"
          >{{ buttonText }}</VBtn
        >
      </VCardActions>
    </VCard>
  </Form>
</template>

<script setup lang="ts">
import { Form, SubmissionHandler, SubmissionContext } from 'vee-validate'

export interface Props {
  title: string
  buttonText: string
  validationSchema: object
  initialValues: Record<string, any>
  onSubmit: SubmissionHandler<any>
  width?: string | number
  successMessage?: string
}
export interface Emits {
  (e: 'onSuccess', value: any): void
}

const props = withDefaults(defineProps<Props>(), {
  width: 500,
  successMessage: undefined,
})
const emit = defineEmits<Emits>()

const computedWidth = computed(() =>
  typeof props.width === 'number' ? `${props.width}px` : props.width
)

const message = useLocaleMessage()

const handleSubmit = async (
  values: Record<string, unknown>,
  ctx: SubmissionContext
) => {
  const result = await props.onSubmit(values, ctx)
  emit('onSuccess', result)
  if (props.successMessage) {
    message.value = {
      type: 'success',
      message: props.successMessage,
    }
  }
}
</script>
