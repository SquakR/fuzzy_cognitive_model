<template>
  <footer class="footer">
    <div class="content has-text-centered">
      <p>
        <span v-html="t('copyright', { years })" />
        <br />
        <span>{{ t('version', { version }) }}</span>
      </p>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const version = ref('0.1.0')
const years = ref('2022-2023')
</script>

<i18n locale="en-ES" lang="json">
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
