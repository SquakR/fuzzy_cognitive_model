<template>
  <VAppBar color="indigo-darken-1" title="Fuzzy Cognitive Model">
    <template #append>
      <VTooltip v-model="tooltipActive" location="bottom">
        <template #activator="{ props: tooltipProps }">
          <VMenu>
            <template #activator="{ props: menuProps }">
              <VBtn
                v-bind="{ ...tooltipProps, ...menuProps }"
                icon="mdi-translate"
                @click="tooltipActive = false"
              ></VBtn>
            </template>
            <VSheet>
              <VList
                v-model:selected="selectedLocale"
                :items="locales"
                density="compact"
              >
                <VListSubheader class="font-weight-bold" color="black">{{
                  t('languages')
                }}</VListSubheader>
                <VListItem
                  v-for="locale in locales"
                  :key="locale.value"
                  :value="locale.value"
                >
                  <VListItemSubtitle style="color: black">{{
                    locale.text
                  }}</VListItemSubtitle>
                </VListItem>
              </VList>
            </VSheet>
          </VMenu>
        </template>
        <span>{{ t('languages') }}</span>
      </VTooltip>
    </template>
  </VAppBar>
</template>

<script setup lang="ts">
import { useLocaleStore } from '~/store'
import { useI18n } from 'vue-i18n'

const { t } = useI18n({})

const tooltipActive = ref(false)

const locales = [
  { text: 'English', value: 'en-US' },
  { text: 'Russian', value: 'ru-RU' },
]
const storeLocale = useLocaleStore()
const selectedLocale = computed({
  get: () => [storeLocale.locale],
  set: (value) => {
    storeLocale.locale = value[0]
  },
})
</script>

<i18n locale="en-US" lang="json">
{
  "languages": "Languages"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "languages": "Языки"
}
</i18n>
