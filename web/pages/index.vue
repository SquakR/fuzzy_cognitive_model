<template>
  <form class="mt-2" style="width: 500px" @submit.prevent="fetchUser()">
    <div class="card">
      <header class="card-header">
        <p class="card-header-title">{{ t('userView') }}</p>
      </header>
      <div class="card-content">
        <BaseTextField
          v-model="userId"
          :label="t('userIdLabel')"
          :placeholder="t('userIdPlaceholder')"
        ></BaseTextField>
        <pre v-if="user">{{ t('user') }} {{ user }}</pre>
        <div v-else>{{ t('userNotLoaded') }}</div>
      </div>
      <footer class="card-footer">
        <div class="card-footer-item is-flex is-justify-content-flex-end">
          <BaseButton type="submit" :loading="userLoading">{{
            t('fetchUser')
          }}</BaseButton>
        </div>
      </footer>
    </div>
  </form>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
const userId = ref('')
const {
  pending: userLoading,
  data: user,
  execute: fetchUser,
} = useAPIUser(computed(() => Number.parseInt(userId.value)))
</script>

<i18n locale="en-ES" lang="json">
{
  "userView": "User View",
  "userIdLabel": "User id",
  "userIdPlaceholder": "Enter user id",
  "user": "User:",
  "userNotLoaded": "User not loaded",
  "fetchUser": "Fetch user"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "userView": "Просмотр пользователя",
  "userIdLabel": "Идентификатор пользователя",
  "userIdPlaceholder": "Введите идентификатор пользователя",
  "user": "Пользователь:",
  "userNotLoaded": "Пользователя не загружен",
  "fetchUser": "Загрузить пользователя"
}
</i18n>
