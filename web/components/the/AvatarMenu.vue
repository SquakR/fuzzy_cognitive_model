<template>
  <VMenu>
    <template #activator="{ props }">
      <VBtn size="large" v-bind="props" icon>
        <UserAvatar
          v-if="modelValue"
          :user="modelValue"
          size="large"
          color="white"
          class="text-indigo-darken-1"
        />
      </VBtn>
    </template>
    <VSheet>
      <VList density="compact">
        <VListItem
          :to="{ name: 'projects' }"
          prepend-icon="mdi-vector-polygon-variant"
        >
          <VListItemTitle>{{ t('projects') }}</VListItemTitle>
        </VListItem>
        <VListItem prepend-icon="mdi-logout" @click="signOut">
          <VListItemTitle>{{ t('signOut') }}</VListItemTitle>
        </VListItem>
      </VList>
    </VSheet>
  </VMenu>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { UserOutType } from '~/types'

export interface Props {
  modelValue: UserOutType | null
}
export interface Emits {
  (e: 'update:modelValue', modelValue: UserOutType | null): void
}
defineProps<Props>()
const emit = defineEmits<Emits>()

const { t } = useI18n()

const { execute: signOut, onSuccess } = useSignOut({
  key: 'signOut',
})
onSuccess(async () => {
  emit('update:modelValue', null)
  await navigateTo({ name: 'auth-sign_in' })
})
</script>

<i18n locale="en-US" lang="json">
{
  "projects": "Projects",
  "signOut": "Sign out"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "projects": "Проекты",
  "signOut": "Выйти"
}
</i18n>
