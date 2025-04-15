<template>
  <MenuBackLayout :title="$t('ui.appeareance.appeareance')" back>
  </MenuBackLayout>
</template>
<script>
import { useI18n } from "vue-i18n";
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
export default {
  setup() {
    const t = useI18n();
    return t;
  },
};
</script>
