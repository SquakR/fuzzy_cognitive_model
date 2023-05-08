<template>
  <BaseBreadcrumbs :items="bc">
    <div class="d-flex justify-center">
      <BaseForm
        :title="t('title')"
        :button-text="t('buttonText')"
        :validation-schema="validationSchema"
        :initial-values="initialValues"
        :on-submit="onSubmit"
        @on-success="onSuccess"
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
  </BaseBreadcrumbs>
</template>

<script setup lang="ts">
import { BreadcrumbItem } from '~/types/base-breadcrumbs'
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'

const { t } = useI18n({})

const bc = computed<BreadcrumbItem[]>(() => [
  {
    title: t('title'),
    to: { name: 'auth-sign_up' },
  },
])

const validationSchema = yup.object({
  username: yup.string().required().min(3),
  email: yup.string().required().email(),
  firstName: yup.string().required().min(2),
  secondName: yup.string().notRequired().min(2),
  lastName: yup.string().required().min(2),
  avatar: yup.mixed(),
  password: yup.string().required().min(8),
  passwordConfirmation: yup
    .string()
    .required()
    .min(8)
    .oneOf([yup.ref('password')]),
})
const initialValues: yup.InferType<typeof validationSchema> = {
  username: '',
  email: '',
  firstName: '',
  lastName: '',
  password: '',
  passwordConfirmation: '',
}
const createUser = useCreateUser()
const onSubmit = async (values: yup.InferType<typeof validationSchema>) => {
  await createUser({
    username: values.username,
    email: values.email,
    firstName: values.firstName,
    secondName: values.secondName ? values.secondName : null,
    lastName: values.lastName,
    avatar: values.avatar ? values.avatar : null,
    password: values.password,
    language: null,
  })
}
const onSuccess = async () => {
  await navigateTo({ name: 'auth-sign_in' })
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
