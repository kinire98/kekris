<template>
  <MenuBackLayout :title="$t('ui.settings.settings')" back> </MenuBackLayout>
</template>

<script lang="ts">
import { useI18n } from "vue-i18n";
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
export default {
  setup() {
    const t = useI18n();
    return t;
  },
};
</script>
