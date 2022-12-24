<template>
  <Form
    v-slot="{ isSubmitting }"
    :style="{ width: typeof width === 'number' ? `${width}px` : width }"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
  >
    <div class="card">
      <header class="card-header">
        <slot name="header">
          <p class="card-header-title">{{ title }}</p>
        </slot>
      </header>
      <div class="card-content">
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
import { Form, SubmissionHandler } from 'vee-validate'
import * as yup from 'yup'

export interface Props {
  width: string | number
  validationSchema: ReturnType<typeof yup.object>
  initialValues: Record<string, string>
  onSubmit: SubmissionHandler
  title?: string
  submitText?: string
}

defineProps<Props>()
</script>
