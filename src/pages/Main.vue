<template>
  <MenuBackLayout>
    <Toast class="not-yet-implemented" position="bottom-right" />
    <MenuButton
      :label="$t('ui.main.singleplayer')"
      :desc="$t('ui.main.singleplayer-desc')"
      path="/singleplayer"
    />
    <MenuButton
      :label="$t('ui.main.multiplayer')"
      :desc="$t('ui.main.multiplayer-desc')"
      path="/multiplayer"
    />
    <MenuButton
      :label="$t('ui.main.settings')"
      :desc="$t('ui.main.settings-desc')"
      path="/settings"
    />
  </MenuBackLayout>
</template>
<script lang="ts">
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
import MenuButton from "../components/MenuButton.vue";
import { Toast } from "primevue";
import { useI18n } from "vue-i18n";
export default {
  setup() {
    const t = useI18n();
    return {
      t,
    };
  },
};
</script>
