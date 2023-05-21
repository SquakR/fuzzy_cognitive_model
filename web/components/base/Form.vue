<template>
  <Form
    ref="form"
    v-slot="{ isSubmitting }"
    as="v-form"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
  >
    <VCard :width="computedWidth" :flat="flat">
      <VCardTitle v-if="title">
        <slot name="title" :title="title">{{ title }}</slot>
      </VCardTitle>
      <VCardSubtitle v-if="subtitle">
        <slot name="subtitle" :subtitle="subtitle">{{ subtitle }}</slot>
      </VCardSubtitle>
      <VCardText>
        <VAlert
          v-if="success"
          color="success"
          class="mb-2"
          closable
          @click:close="success = null"
          >{{ success }}</VAlert
        >
        <VAlert
          v-else-if="error"
          color="error"
          class="mb-2"
          closable
          @click:close="error = null"
          >{{ error }}</VAlert
        >
        <slot />
      </VCardText>
      <VCardActions>
        <slot
          name="actions"
          :disabled="disabled"
          :loading="isSubmitting"
          :button-text="buttonText"
        >
          <VSpacer />
          <VBtn
            :disabled="disabled"
            :loading="isSubmitting"
            color="primary"
            variant="elevated"
            type="submit"
            >{{ buttonText }}</VBtn
          >
        </slot>
      </VCardActions>
    </VCard>
  </Form>
</template>

<script setup lang="ts">
import { Form, SubmissionHandler } from 'vee-validate'
import { useMessageStore } from '~/store'

export interface Props {
  actionKey: string | string[]
  buttonText: string
  validationSchema: object
  initialValues: Record<string, any>
  onSubmit: SubmissionHandler<any>
  disabled?: boolean
  title?: string
  subtitle?: string
  width?: string | number
  flat?: boolean
  successMessage?: string
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  title: undefined,
  subtitle: undefined,
  width: 500,
  flat: false,
  successMessage: undefined,
})

const form = ref<InstanceType<typeof Form> | null>(null)
defineExpose({
  form,
})

const computedWidth = computed(() =>
  typeof props.width === 'number' ? `${props.width}px` : props.width
)

const messageStore = useMessageStore()
const { success, error, unsubscribe } = messageStore.subscribeLocal(
  props.actionKey
)
onUnmounted(() => {
  unsubscribe()
})
</script>
