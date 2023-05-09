<template>
  <Form
    v-slot="{ isSubmitting }"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
  >
    <VCard :width="computedWidth">
      <VCardTitle>
        <slot name="title" :title="title">{{ title }}</slot>
      </VCardTitle>
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
import { Form, SubmissionHandler } from 'vee-validate'
import { useMessageStore } from '~/store'

export interface Props {
  actionKey: string
  title: string
  buttonText: string
  validationSchema: object
  initialValues: Record<string, any>
  onSubmit: SubmissionHandler<any>
  width?: string | number
  successMessage?: string
}

const props = withDefaults(defineProps<Props>(), {
  width: 500,
  successMessage: undefined,
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
