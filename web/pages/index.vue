<template>
  <BaseForm
    class="mt-2"
    :width="600"
    :title="t('userView')"
    :loading="userLoading"
    :submit-text="t('fetchUser')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onUserSubmit"
  >
    <template #content>
      <BaseTextField
        v-model="userId"
        name="userId"
        :label="t('userId')"
        :placeholder="t('userIdPlaceholder')"
      ></BaseTextField>
      <div style="height: 188px">
        <pre v-if="user">{{ t('user') }} {{ user }}</pre>
        <div v-else>{{ t('userNotLoaded') }}</div>
      </div>
    </template>
  </BaseForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import { UseAPIUserVariables } from '~~/composables/api'

const { t } = useI18n()

const validationSchema = yup.object({
  userId: yup.number().required().min(1).max(10000),
})
const initialValues = {
  userId: '',
}

const userId = ref('')
const { pending: userLoading, data: user, execute: fetchUser } = useAPIUser()
const onUserSubmit = async (values: Record<string, unknown>) => {
  await fetchUser(values as UseAPIUserVariables)
}
</script>

<i18n locale="en-ES" lang="json">
{
  "userView": "User View",
  "userId": "User id",
  "userIdPlaceholder": "Enter user id",
  "user": "User:",
  "userNotLoaded": "User not loaded",
  "fetchUser": "Fetch user"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "userView": "Просмотр пользователя",
  "userId": "Идентификатор пользователя",
  "userIdPlaceholder": "Введите идентификатор пользователя",
  "user": "Пользователь:",
  "userNotLoaded": "Пользователя не загружен",
  "fetchUser": "Загрузить пользователя"
}
</i18n>
