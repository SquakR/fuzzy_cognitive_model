<template>
  <Form
    v-slot="{ isSubmitting }"
    :style="{ width: typeof width === 'number' ? `${width}px` : width }"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="handleSubmit"
  >
    <div class="card">
      <header class="card-header">
        <slot name="header">
          <p class="card-header-title">{{ title }}</p>
        </slot>
      </header>
      <div class="card-content">
        <div
          v-if="notification"
          class="notification"
          :class="notificationClasses"
        >
          <button class="delete" @click="close" />
          {{ message }}
        </div>
        <slot name="content"></slot>
      </div>
      <footer class="card-footer">
        <slot name="footer">
          <div class="card-footer-item is-flex is-justify-content-flex-end">
            <BaseButton type="submit" :loading="isSubmitting">{{
              submitText
            }}</BaseButton>
          </div>
        </slot>
      </footer>
    </div>
  </Form>
</template>

<script setup lang="ts">
import { FetchError } from 'ofetch'
import { SubmissionContext } from 'vee-validate'
import { Form, SubmissionHandler } from 'vee-validate'
import * as yup from 'yup'

export interface Props {
  width: string | number
  validationSchema: ReturnType<typeof yup.object>
  initialValues: Record<string, string>
  onSubmit: SubmissionHandler
  title?: string | null
  submitText?: string | null
  errorInterval?: number
  successInterval?: number
  successMessage?: string | null
}

const props = withDefaults(defineProps<Props>(), {
  title: null,
  submitText: null,
  errorInterval: Infinity,
  successInterval: 0,
  successMessage: null,
})

const {
  notification,
  message,
  setSuccessNotification,
  setFetchErrorNotification,
  deleteNotification,
} = useNotification()

const notificationClasses = computed(() => ({
  'is-success': notification.value?.type === 'success',
  'is-danger': notification.value?.type === 'fetchError',
}))

const close = () => {
  deleteNotification()
}

const handleSubmit = async (
  values: Record<string, unknown>,
  ctx: SubmissionContext
) => {
  try {
    await props.onSubmit(values, ctx)
    if (props.successMessage) {
      setSuccessNotification(props.successMessage, props.successInterval)
    } else {
      deleteNotification()
    }
  } catch (e) {
    if (e instanceof FetchError) {
      setFetchErrorNotification(e, Infinity)
    } else {
      throw e
    }
  }
}
</script>
