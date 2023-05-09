<template>
  <VTooltip v-model="tooltipActive" location="bottom">
    <template #activator="{ props: tooltipProps }">
      <VMenu>
        <template #activator="{ props: menuProps }">
          <VBtn
            v-bind="{ ...$attrs, ...tooltipProps, ...menuProps }"
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

<script lang="ts">
export default defineComponent({
  inheritAttrs: false,
})
</script>

<script setup lang="ts">
import { useUserStore } from '~/store'
import { useI18n } from 'vue-i18n'

const { t } = useI18n({})

const userStore = useUserStore()

const tooltipActive = ref(false)

const locales = [
  { text: 'English', value: 'en-US' },
  { text: 'Russian', value: 'ru-RU' },
]
const selectedLocale = computed({
  get: () => [userStore.locale],
  set: (value) => {
    userStore.locale = value[0]
  },
})
onMounted(() => {
  userStore.updateLocale()
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
