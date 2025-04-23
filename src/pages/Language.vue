<template>
  <MenuBackLayout :title="$t('ui.language.language')" back>
    <div v-for="locale in availableLocales" class="flag">
      <Locale :locale="locale" />
    </div>
  </MenuBackLayout>
</template>
<style scoped>
.flag {
  margin-block: 50px;
}
</style>
<script setup lang="ts">
useI18n();
const { availableLocales } = useI18n();
</script>
<script lang="ts">
import { useI18n } from "vue-i18n";
import Locale from "../components/Locale.vue";
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
export default {};
</script>
