<template>
  <div class="root-room">
    <h3>{{ props.info.name }}</h3>
    <div id="info">
      <span>
        {{ props.info.number_of_players }}/{{ props.info.limit_of_players }}
        <Icon icon="proicons:person" />
      </span>
      <span>
        {{ props.info.games_played }}
        <span v-if="props.info.games_played == 1">
          {{ $t("ui.multiplayer.local.game") }}
        </span>
        <span v-else>{{ $t("ui.multiplayer.local.games") }}</span>
      </span>
    </div>
  </div>
</template>
<style scoped>
.root-room {
  display: flex;
  align-items: start;
  justify-content: center;
  flex-direction: column;
  width: 450px;
  height: 150px;
  margin-inline: 50px;
  margin-block: 15px;
  padding: 15px;
  padding-bottom: 30px;
  padding-inline: 20px;
  background-color: #30303066;
  border-radius: 15px;
  cursor: pointer;
  transition: all 0.3s;
}
.root-room:hover {
  transform: scale(1.1);
}
h3 {
  font-size: 22px;
}
#info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-direction: row;
  width: 100%;
  font-size: 18px;
}
</style>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { RoomInfo } from "../types/Room";
import { Icon } from "@iconify/vue/dist/iconify.js";
useI18n();

const props = defineProps<{
  info: RoomInfo;
}>();
</script>
