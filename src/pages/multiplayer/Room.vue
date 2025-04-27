<template>
  <div id="root">
    <div id="top-bar">
      <Button
        label="<-"
        id="back"
        variant="outlined"
        raised
        @click="leaveRoom"
      />
      <h1>
        {{ roomName }}
      </h1>
    </div>
    <div id="content">
      <div class="room-content">
        <h2>
          {{ $t("ui.multiplayer.room.players") }}
          {{ room.players.length + 1 }} /
          {{ room.limit_of_players + 1 }}
        </h2>
        <div id="player-list">
          <PlayerComponent
            :name="getUsername()!"
            :kick="false"
            :games_won="0"
            :games_played="room.games_played"
          />
          <PlayerComponent
            v-for="player in players"
            :name="player.name"
            :kick="false"
            :games_played="room.games_played"
            :games_won="0"
          />
        </div>
      </div>
      <div id="room-controls" v-if="name == 'host'">
        <Button
          :label="$t('ui.multiplayer.room.play')"
          variant="outlined"
          id="play"
          @click="startGame"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
#root {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}
#top-bar {
  display: flex;
  align-items: center;
  justify-content: start;
  padding: 50px;
  height: 15vh;
  width: 100vw;
}
#content {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
  height: 85vh;
  width: 100vw;
  > div {
    width: 50vw;
  }
}
.room-content {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  height: 100%;
}
#player-list {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  height: 80%;
  width: 100%;
}
#room-controls {
  display: flex;
  align-items: center;
  justify-content: center;
}
#play {
  font-size: 35px;
  padding-inline: 250px;
  padding-block: 25px;
}
#back {
  width: 5vw;
  height: 50px;
  border-color: var(--main-color);
  color: var(--main-color);
  margin-block: 50px;
  font-size: 25px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
}
#back:hover {
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
}
.p-button-outlined {
  border: 2.5px solid var(--main-color) !important;
  color: var(--main-color) !important;
  margin-block: 10px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: start;
}
.p-button-outlined:hover {
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
}
h1 {
  margin-left: 50px;
  transition: all 0.3s;
}
</style>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMounted, Ref, ref } from "vue";
import { useRoute } from "vue-router";
import { Player, Room } from "../../types/Room";
import { Button } from "primevue";
import { getUsername } from "../../helpers/username";
import { useI18n } from "vue-i18n";
import i18n from "../../i18n";
import { default as PlayerComponent } from "../../components/Player.vue";

const playersEmit = "playersEmit";
const roomNameEmit = "roomNameEmit";

useI18n();

let route = useRoute();
let name = route.path.substring(1);
let visible: boolean = false;
let roomName: string =
  name == "host"
    ? i18n.global.t("ui.multiplayer.room.room-of") + " " + getUsername()
    : await invoke("get_room_name");
const players: Ref<Player[]> = ref([]);
let room: Room;
if (name == "host") {
  room = (await invoke("create_room", { name: roomName })) as Room;
  players.value = room.players;
} else {
  room = (await invoke("room_info")) as Room;
}
listen(playersEmit, (e) => {
  players.value = e.payload as Player[];
});
function leaveRoom() {
  if (name == "host") {
    invoke("close_room");
  } else {
    invoke("leave_room");
  }
}
function startGame() {}
</script>
