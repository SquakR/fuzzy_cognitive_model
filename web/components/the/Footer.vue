<template>
  <VFooter class="the-footer__footer" color="indigo" app>
    <div v-html="t('copyright', { years })" />
    <div>{{ t('version', { version: packageJson.version }) }}</div>
  </VFooter>
</template>

<script setup lang="ts">
import packageJson from '../../package.json'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const years = `2022-${new Date().getFullYear()}`
</script>

<i18n locale="en-US" lang="json">
{
  "copyright": "Copyright &#169; Orlov Artem {years}",
  "version": "Version: {version}"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "copyright": "Copyright &#169; Орлов Артем {years}",
  "version": "Версия: {version}"
}
</i18n>

<style lang="sass">
.the-footer__footer
  width: 100%
  display: flex
  flex-direction: column
  align-items: center
</style>
