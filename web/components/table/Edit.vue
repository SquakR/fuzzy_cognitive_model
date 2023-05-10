<template>
  <VTooltip v-model="tooltipActive" location="bottom">
    <template #activator="{ props }">
      <VIcon v-bind="{ ...$attrs, ...props }" color="primary">
        mdi-pencil
      </VIcon>
    </template>
    <span>{{ t('change') }}</span>
  </VTooltip>
</template>

<script lang="ts">
export default defineComponent({
  inheritAttrs: false,
})
</script>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const tooltipActive = ref(false)
</script>

<i18n locale="en-US" lang="json">
{
  "change": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "change": "Изменить"
}
</i18n>
