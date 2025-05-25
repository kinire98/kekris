<template>
  <MenuBackLayout back :title="$t('ui.multiplayer.multiplayer')">
    <Toast class="unavailable" position="bottom-right" />
    <MenuButton
      :label="$t('ui.multiplayer.local-network')"
      :desc="$t('ui.multiplayer.local-network-desc')"
      path="/local"
    />
    <MenuButton
      :label="$t('ui.multiplayer.internet')"
      :desc="$t('ui.multiplayer.internet-desc')"
      path="/internet"
    />
  </MenuBackLayout>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import MenuButton from "../../components/MenuButton.vue";
import MenuBackLayout from "../../layouts/MenuBackLayout.vue";
import { onBeforeRouteLeave } from "vue-router";
import { RouteLocationNormalized } from "vue-router";
import { RouteLocationNormalizedLoaded } from "vue-router";
import { NavigationGuardNext } from "vue-router";
import { useToast } from "primevue";
import { Toast } from "primevue";
import i18n from "../../i18n";
useI18n();
const toast = useToast();
onBeforeRouteLeave(
  (
    to: RouteLocationNormalized,
    _from: RouteLocationNormalizedLoaded,
    next: NavigationGuardNext
  ) => {
    if (to.path == "/internet") {
      toast.add({
        severity: "contrast",
        life: 2500,
        summary: i18n.global.t("ui.multiplayer.not-available"),
      });
      next(false);
    } else {
      next(true);
    }
  }
);
</script>
