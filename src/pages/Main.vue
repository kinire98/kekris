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
<script lang="ts" setup>
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
import MenuButton from "../components/MenuButton.vue";
import { Toast, useToast } from "primevue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { onBeforeRouteLeave } from "vue-router";
import i18n from "../i18n";
useI18n();
const toast = useToast();
let show = await invoke("can_host_room");
onBeforeRouteLeave((to, _from, next) => {
  if (!show && to.path == "/multiplayer") {
    toast.add({
      severity: "contrast",
      life: 2500,
      summary: i18n.global.t("ui.main.not-available-mac"),
    });
    next(false);
    return;
  }
  next(true);
});
</script>
