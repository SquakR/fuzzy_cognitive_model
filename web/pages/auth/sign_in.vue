<template>
  <BaseBreadcrumbs :items="bc">
    <div class="d-flex justify-center"></div>
  </BaseBreadcrumbs>
</template>

<script setup lang="ts">
import { BreadcrumbItem } from '~/types/base-breadcrumbs'
import { useI18n } from 'vue-i18n'

const { t } = useI18n({})

const bc = computed<BreadcrumbItem[]>(() => [
  {
    title: t('title'),
    to: { name: 'auth-sign_up' },
  },
])
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Sign In"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Авторизация"
}
</i18n>
