<template>
  <div class="d-flex justify-center mt-5">
    <BaseForm
      :action-key="ACTION_KEY"
      :title="t('title')"
      :button-text="t('buttonText')"
      :validation-schema="validationSchema"
      :initial-values="initialValues"
      :on-submit="onSubmit"
    >
      <BaseTextField :label="t('username')" name="username" />
      <BaseTextField :label="t('email')" name="email" type="email" />
      <BaseTextField :label="t('firstName')" name="firstName" />
      <BaseTextField :label="t('secondName')" name="secondName" clearable />
      <BaseTextField :label="t('lastName')" name="lastName" />
      <BaseFileInput :label="t('avatar')" name="avatar" accept="image/*" />
      <BasePasswordField :label="t('password')" name="password" />
      <BasePasswordField
        :label="t('passwordConfirmation')"
        name="passwordConfirmation"
      />
    </BaseForm>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import { useUserStore } from '~/store'

definePageMeta({
  middleware: ['guest'],
})

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

const ACTION_KEY = 'signUp'
const validationSchema = $yup.object({
  username: $yup.string().required().min(3).max(255),
  email: $yup.string().required().email().max(255),
  firstName: $yup.string().required().min(2).max(255),
  secondName: $yup.string().notRequired().min(2).max(255),
  lastName: $yup.string().required().min(2).max(255),
  avatar: $yup.mixed(),
  password: $yup.string().required().min(8),
  passwordConfirmation: $yup
    .string()
    .required()
    .min(8)
    .oneOf([$yup.ref('password')]),
})
const initialValues: yup.InferType<typeof validationSchema> = {
  username: '',
  email: '',
  firstName: '',
  lastName: '',
  password: '',
  passwordConfirmation: '',
}
const { execute: createUser, onSuccess } = useCreateUser({
  key: ACTION_KEY,
})
onSuccess(async () => {
  await navigateTo({ name: 'auth-sign_in' })
})
const onSubmit = async (values: yup.InferType<typeof validationSchema>) => {
  await createUser({
    username: values.username,
    email: values.email,
    firstName: values.firstName,
    secondName: values.secondName ? values.secondName : null,
    lastName: values.lastName,
    avatar: values.avatar ? values.avatar : null,
    password: values.password,
    locale: userStore.locale,
  })
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Sign Up",
  "buttonText": "Sign up",
  "username": "Username",
  "email": "Email",
  "firstName": "First Name",
  "secondName": "Second Name",
  "lastName": "Last Name",
  "avatar": "Avatar",
  "password": "Password",
  "passwordConfirmation": "Password Confirmation"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Регистрация",
  "buttonText": "Зарегистрироваться",
  "username": "Логин",
  "email": "Email",
  "firstName": "Имя",
  "secondName": "Отчество",
  "lastName": "Фамилия",
  "avatar": "Аватар",
  "password": "Пароль",
  "passwordConfirmation": "Подтверждение пароля"
}
</i18n>
