<template>
  <MenuBackLayout :title="$t('ui.singleplayer.singleplayer')" back>
    <MenuButton
      :label="$t('ui.singleplayer.classic')"
      :desc="$t('ui.singleplayer.classic-desc')"
      path="/classic"
    />
    <MenuButton
      :label="$t('ui.singleplayer.lines')"
      :desc="$t('ui.singleplayer.lines-desc')"
      path="/lines"
    />
    <MenuButton
      :label="$t('ui.singleplayer.blitz')"
      :desc="$t('ui.singleplayer.blitz-desc')"
      path="/blitz"
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
