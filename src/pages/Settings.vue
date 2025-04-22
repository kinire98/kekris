<template>
  <MenuBackLayout :title="$t('ui.settings.settings')" back>
    <MenuButton
      :label="$t('ui.settings.controls')"
      :desc="$t('ui.settings.controls-desc')"
      path="/settings/controls"
    />
    <MenuButton
      :label="$t('ui.settings.appeareance')"
      :desc="$t('ui.settings.appeareance-desc')"
      path="/settings/appeareance"
    />
    <MenuButton
      :label="$t('ui.settings.language')"
      :desc="$t('ui.settings.language-desc')"
      path="/settings/language"
    />
  </MenuBackLayout>
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
