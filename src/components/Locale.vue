<template>
  <div @click="changeLocale">{{ translateToEmoji(props.locale!) }}</div>
</template>
<style scoped>
div {
  font-size: 75px;
  cursor: pointer;
  transition: all 0.3s;
}
div:hover {
  transform: scale(1.2);
}
</style>
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { setDefaultLocale } from "../helpers/defaultLocale";

const props = defineProps<{
  locale: string;
}>();

const { locale } = useI18n();

const languageToCountryMap: Record<string, string> = {
  en: "GB",
  es: "ES",
  fr: "FR",
  de: "DE",
  it: "IT",
  pt: "BR",
  ja: "JP",
  zh: "CN",
  ru: "RU",
};

function translateToEmoji(countryCode: string): string {
  return languageToCountryMap[countryCode]
    .toUpperCase()
    .replace(/./g, (char) => String.fromCodePoint(127397 + char.charCodeAt(0)));
}

function changeLocale() {
  locale.value = props.locale;
  setDefaultLocale(props.locale);
}
</script>
