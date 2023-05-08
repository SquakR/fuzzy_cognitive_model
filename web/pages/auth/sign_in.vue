<template>
  <BaseBreadcrumbs :items="bc">
    <div class="d-flex justify-center">
      <BaseForm
        :action-key="actionKey"
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
  </BaseBreadcrumbs>
</template>

<script setup lang="ts">
import { BreadcrumbItem } from '~/types/base-breadcrumbs'
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'

definePageMeta({
  middleware: ['guest'],
})

const { t } = useI18n({})

const bc = computed<BreadcrumbItem[]>(() => [
  {
    title: t('title'),
    to: { name: 'auth-sign_in' },
  },
])

const actionKey = 'signIn'
const validationSchema = yup.object({
  username: yup.string().required().min(3),
  password: yup.string().required().min(8),
})
const initialValues: yup.InferType<typeof validationSchema> = {
  username: '',
  password: '',
}
const signIn = useSignIn({
  key: actionKey,
  onSuccess: async () => {
    await navigateTo({ name: 'index' })
  },
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
