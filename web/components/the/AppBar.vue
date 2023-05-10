<template>
  <VAppBar color="indigo-darken-1" title="Fuzzy Cognitive Model">
    <template #append>
      <VTooltip location="bottom">
        <template #activator="{ props }">
          <VBtn
            v-bind="props"
            href="https://github.com/SquakR/fuzzy_cognitive_model"
            target="_blank"
            class="mr-1"
            icon
          >
            <VIcon size="x-large">mdi-github</VIcon>
          </VBtn>
        </template>
        <span>{{ t('repository') }}</span>
      </VTooltip>
      <TheLocaleMenu class="mr-1" />
      <TheAvatarMenu v-if="userStore.user" v-model="userStore.user" />
      <template v-else>
        <VBtn :to="{ name: 'auth-sign_in' }">{{ t('signIn') }}</VBtn>
        <VBtn :to="{ name: 'auth-sign_up' }">{{ t('signUp') }}</VBtn>
      </template>
    </template>
  </VAppBar>
</template>

<script setup lang="ts">
import { useUserStore } from '~/store'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const userStore = useUserStore()
</script>

<i18n locale="en-US" lang="json">
{
  "repository": "Repository",
  "signIn": "Sign in",
  "signUp": "Sign up"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "repository": "Репозиторий",
  "signIn": "Вход",
  "signUp": "Регистрация"
}
</i18n>
