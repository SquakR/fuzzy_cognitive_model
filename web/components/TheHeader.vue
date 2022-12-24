<template>
  <header>
    <nav class="navbar is-fixed-top is-primary">
      <div class="navbar-brand">
        <h1
          class="title has-text-white-ter is-flex is-align-items-center mb-1 ml-3"
        >
          Fuzzy Cognitive Model
        </h1>
        <a
          role="button"
          class="navbar-burger"
          aria-label="menu"
          aria-expanded="false"
          :class="{ 'is-active': isMenuActive }"
          @click="isMenuActive = !isMenuActive"
        >
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>
      <div :class="menuClasses">
        <template v-if="isMenuActive">
          <a
            :aria-expanded="isLanguageActive"
            class="panel-block is-flex is-justify-content-space-between"
            role="button"
            aria-level="1"
            aria-controls="en ru"
            @click="isLanguageActive = !isLanguageActive"
            ><span>{{ t('language') }}</span>
            <FontAwesomeIcon
              :icon="
                isLanguageActive
                  ? 'fa-solid fa-angle-up'
                  : 'fa-solid fa-angle-down'
              "
            />
          </a>
          <template v-if="isLanguageActive">
            <a
              v-for="language in languages"
              :key="language"
              :class="{ 'has-text-primary': locale === language }"
              class="panel-block pl-5"
              role="button"
              aria-level="2"
              @click="locale = language"
              >{{ t(language) }}</a
            >
          </template>
        </template>
        <div v-else class="navbar-end">
          <div
            class="navbar-item has-dropdown is-hoverable mr-3"
            role="listbox"
            aria-labelledby="language-label"
          >
            <a id="language-label" class="navbar-link"> {{ t('language') }} </a>
            <div class="navbar-dropdown is-boxed">
              <a
                v-for="language in languages"
                :key="language"
                class="navbar-item"
                :class="{ 'has-text-primary': locale === language }"
                role="option"
                @click="locale = language"
                >{{ t(language) }}</a
              >
            </div>
          </div>
        </div>
      </div>
    </nav>
  </header>
</template>

<script setup lang="ts">
import { useWindowSize } from '@vueuse/core'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const { locale } = useI18n({ useScope: 'global' })

const isMenuActive = ref(false)
const isLanguageActive = ref(false)

const { width: windowWidth } = useWindowSize()
watch(windowWidth, (newValue) => {
  if (newValue > 1024) {
    isMenuActive.value = false
    isLanguageActive.value = false
  }
})

const menuClasses = computed(() => {
  const classes = ['navbar-menu']
  if (isMenuActive.value) {
    classes.push('is-active', 'panel', 'the-header__menu_active')
  }
  return classes
})

const languages = ['en-ES', 'ru-RU']
</script>

<i18n locale="en-ES" lang="json">
{
  "language": "Language",
  "en-ES": "English",
  "ru-RU": "Русский"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "language": "Язык",
  "en-ES": "English",
  "ru-RU": "Русский"
}
</i18n>

<style lang="sass">
.the-header__menu_active
  border-radius: 0
</style>
