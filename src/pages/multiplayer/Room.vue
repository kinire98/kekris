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
          {{ players.length }} /
          {{ room.limit_of_players + 1 }}
        </h2>
        <div id="player-list">
          <PlayerComponent
            v-for="player in players"
            :name="player.name"
            :kick="false"
            :games_played="room.games_played"
            :games_won="0"
            class="players"
          />
        </div>
      </div>
      <div id="room-controls" v-if="name == 'host' || name == 'rehost'">
        <Button
          :label="$t('ui.multiplayer.room.play')"
          variant="outlined"
          id="play"
          @click="startGame"
          :disabled="players.length < 2"
        />
      </div>
    </div>
    <Dialog
      v-model:visible="visiblePopUp"
      modal
      :header="$t('ui.multiplayer.room.ended-header')"
    >
      {{ $t("ui.multiplayer.room.ended") }}
      <Button
        :label="$t('ui.multiplayer.room.ended-button')"
        variant="outlined"
        raised
        @click="popUpClosed"
      ></Button>
    </Dialog>
    <Dialog
      v-model:visible="visiblePopUpConnection"
      modal
      :header="$t('ui.multiplayer.room.ended-connection-header')"
    >
      {{ $t("ui.multiplayer.room.ended-connection") }}
      <Button
        :label="$t('ui.multiplayer.room.ended-button')"
        variant="outlined"
        raised
        @click="popUpConnectionClosed"
      ></Button>
    </Dialog>
    <Toast class="unavailable" position="bottom-right" />
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
  justify-content: start;
  flex-direction: column;
  height: 60%;
  width: 100%;
  overflow-y: scroll;
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
.players {
  margin-block: 15px;
}
</style>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMounted, Ref, ref, watch } from "vue";
import { useRoute } from "vue-router";
import { Player, Room, Visibility } from "../../types/Room";
import { Button, useToast } from "primevue";
import { getUsername } from "../../helpers/username";
import { useI18n } from "vue-i18n";
import i18n from "../../i18n";
import { default as PlayerComponent } from "../../components/Player.vue";
import { router } from "../../router";
import { Toast } from "primevue";
import { Dialog } from "primevue";

const playersEmit = "playersEmit";
const roomNameEmit = "roomNameEmit";
const roomClosedEmit = "roomClosed";
const lostConnectionEmit = "connectionLost";
const gameStartedEmit = "gameStartedEmit";

useI18n();

let route = useRoute();
let name = route.path.substring(1);
let visible: boolean = false;

let visiblePopUp = ref(false);
let visiblePopUpConnection = ref(false);
let roomName: string =
  name == "host"
    ? i18n.global.t("ui.multiplayer.room.room-of") + " " + getUsername()
    : "";
const players: Ref<Player[]> = ref([]);
const toast = useToast();
let room: Room;
if (name == "host") {
  room = (await invoke("create_room", {
    name: roomName,
    playerName: getUsername()!,
  })) as Room;
  players.value = room.players;
} else if (name == "join") {
  let tmpRoom = (await invoke("room_info")) as Room | null;
  if (tmpRoom == null) {
    router.back();
    room = {
      players: [],
      name: "",
      visibility: "LocalNetwork",
      limit_of_players: 0,
      games_played: 0,
    };
  } else {
    room = tmpRoom!;
    roomName = room.name;
  }
} else {
  console.log("different route");
  room = {
    players: [],
    name: "",
    visibility: "LocalNetwork",
    limit_of_players: 0,
    games_played: 0,
  };
}
listen(playersEmit, (e) => {
  players.value = e.payload as Player[];
});
listen(roomClosedEmit, () => {
  visiblePopUp.value = true;
});
listen(lostConnectionEmit, () => {
  visiblePopUpConnection.value = true;
});
listen(gameStartedEmit, (e) => {
  let id = e.payload as number;
  router.push(`/mutliplayer-board/${players.value.length}/${id}`);
});
function leaveRoom() {
  if (name == "host") {
    invoke("close_room");
  } else {
    invoke("leave_room");
  }
  router.push("/main");
}
function startGame() {
  if (players.value.length >= 2) {
    invoke("start_online_game");
  } else {
    toast.add({
      severity: "contrast",
      life: 2500,
      summary: i18n.global.t("ui.multiplayer.room.wait-for-players"),
    });
  }
}
function popUpClosed() {
  visiblePopUp.value = false;
  router.push("/main");
}
function popUpConnectionClosed() {
  visiblePopUpConnection.value = false;
  router.push("/main");
}
watch(visiblePopUp, (newValue, _oldValue) => {
  if (!newValue) {
    router.push("/main");
  }
});
watch(visiblePopUpConnection, (newValue, _oldValue) => {
  if (!newValue) {
    router.push("/main");
  }
});
</script>
