<template>
  <MenuBackLayout back="false">
    <Toast class="not-yet-implemented" position="bottom-right" />
    <MenuButton label="Single player" path="/singleplayer" />
    <MenuButton label="Multiplayer" path="/multiplayer" />
    <MenuButton label="Settings" path="/settings" />
  </MenuBackLayout>
</template>
<script lang="ts">
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
import MenuButton from "../components/MenuButton.vue";
import { Toast } from "primevue";
import { useToast } from "primevue";
export default {
  beforeRouteLeave(to, _from, next) {
    if (to.path === "/multiplayer") {
      const toast = useToast();
      toast.add({
        severity: "contrast",
        summary: "Multiplayer soon available",
        life: 2500,
      });
      next(false);
    }
    next(true);
  },
};
</script>
