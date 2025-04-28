<template>
  <MenuBackLayout back :title="$t('ui.multiplayer.local.local')">
    <div class="root">
      <div id="host-game">
        <MenuButton
          :label="$t('ui.multiplayer.local.host')"
          :desc="$t('ui.multiplayer.local.host-desc')"
          path="/host"
        />
      </div>
      <div id="games">
        <div v-if="games.length == 0">
          <h2>{{ $t("ui.multiplayer.local.no-rooms") }}</h2>
        </div>
        <div v-else id="rooms">
          <div id="room-listing">
            <Game v-for="game in games" :info="game" @click="join(game)" />
          </div>
        </div>
      </div>
    </div>
  </MenuBackLayout>
</template>
<style scoped>
.root {
  width: 100%;
  height: 100%;
}
#rooms {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  overflow-y: scroll;
  overflow-x: hidden;
}
#room-listing {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  overflow-y: scroll;
  overflow-x: hidden;
  max-height: 65vh;
  padding-bottom: 65px;
}
h2 {
  width: 100%;
  text-align: left;
  margin-bottom: 50px;
  padding-left: 12vw;
}
</style>

<script setup lang="ts">
const roomUpdatesEvent = "room-updates";
import { useI18n } from "vue-i18n";
import MenuBackLayout from "../../layouts/MenuBackLayout.vue";
import { onMounted, Ref, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { RoomInfo } from "../../types/Room";
import { invoke } from "@tauri-apps/api/core";
import MenuButton from "../../components/MenuButton.vue";
import { router } from "../../router";
import { onBeforeRouteLeave } from "vue-router";
useI18n();
const games: Ref<RoomInfo[]> = ref([]);
onMounted(() => {
  invoke("listen_for_rooms");
  listen(roomUpdatesEvent, (e) => {
    games.value = e.payload as RoomInfo[];
  });
});
function join(room: RoomInfo) {
  invoke("join_room", {
    room: room,
  });
  router.push("/join");
}
onBeforeRouteLeave((to, from, next) => {
  invoke("stop_search");
  next(true);
});
</script>
