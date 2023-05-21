<template>
  <div class="d-flex justify-center mt-5">
    <BaseForm
      :action-key="ACTION_KEY"
      :title="t('title')"
      :button-text="t('buttonText')"
      :validation-schema="validationSchema"
      :initial-values="initialValues"
      :on-submit="signIn"
    >
      <BaseTextField :label="t('username')" name="username" />
      <BasePasswordField :label="t('password')" name="password" />
    </BaseForm>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'

definePageMeta({
  middleware: ['guest'],
})

const { $yup } = useNuxtApp()
const { t } = useI18n()

const ACTION_KEY = 'signIn'
const validationSchema = $yup.object({
  username: $yup.string().required().min(3),
  password: $yup.string().required().min(8),
})
const initialValues: yup.InferType<typeof validationSchema> = {
  username: '',
  password: '',
}
const { execute: signIn, onSuccess } = useSignIn({
  key: ACTION_KEY,
})
onSuccess(async () => {
  await navigateTo({ name: 'projects' })
})
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Sign In",
  "buttonText": "Sign in",
  "username": "Username",
  "password": "Password"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Авторизация",
  "buttonText": "Войти",
  "username": "Логин",
  "password": "Пароль"
}
</i18n>
