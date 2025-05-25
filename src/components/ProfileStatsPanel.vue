<template>
  <div class="stats">
    <h1>{{ props.title }}</h1>
    <br />
    <div class="info" v-if="!info.empty">
      <h2>{{ $t("ui.profile.last_game") }}</h2>
      <h3>
        {{
          last_game_info + (points ? " " + $t("ui.profile.points_lower") : "")
        }}
      </h3>
      <div id="top-five">
        <h2>{{ $t("ui.profile.top_5") }}</h2>
        <TimeMark
          v-for="el in timeMarksInfo"
          :mark="el.mark"
          :position="el.position"
          :current="el.its_last_played"
          class="time-mark"
        />
      </div>
    </div>
    <div class="info" v-else>
      <h2 class="no-played">{{ $t("ui.profile.no_info") }}</h2>
    </div>
  </div>
</template>
<style scoped>
div.stats {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  padding: 35px;
  margin: 35px;
  margin-block: 150px;
  border-radius: 15px;
  background-color: #30303066;
  width: 30vw;
  height: 65vh;
  overflow-y: scroll;
  overflow-x: hidden;
}
.no-played {
  text-align: center;
}
.time-mark {
  width: 100%;
}
.top-five,
.info {
  width: 100%;
}
.info h3 {
  margin-bottom: 15px;
}
.top-five {
  transform: translateY(15px);
}
</style>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { EmitGameInfo, isClassic, isLines } from "../types/EmitGameInfo";
import { useI18n } from "vue-i18n";
import { formatSecondsToHHMMSS } from "../helpers/formatSeconds";
import TimeMark from "./TimeMark.vue";
import { TimeMarkInfo } from "../types/TimeMarkInfo";

const props = defineProps<{
  commandName: String;
  title: String;
}>();
useI18n();
let info: EmitGameInfo = await invoke(props.commandName.toString());
let specific_info = info.last_game_info.specific_info;
let last_game_info: string;
let points = true;
if (isClassic(specific_info)) {
  last_game_info = specific_info.Classic.points.toString();
  //   secondsPlayed = specific_info.Classic.time_endured;
  //   returnUrl = "/classic";
} else if (isLines(specific_info)) {
  last_game_info = formatSecondsToHHMMSS(specific_info.Lines.time_endured);
  //   secondsPlayed = specific_info.Lines.time_endured;
  //   returnUrl = "/lines";
  points = false;
} else {
  last_game_info = specific_info.Blitz.points.toString();
  //   secondsPlayed = 120;
  //   returnUrl = "/blitz";
}
let timeMarksInfo: TimeMarkInfo[] = info.top_five_results.map((el, index) => {
  let mark: string;

  if (isClassic(el.specific_info)) {
    mark = el.specific_info.Classic.points.toString();
  } else if (isLines(el.specific_info)) {
    mark = formatSecondsToHHMMSS(el.specific_info.Lines.time_endured);
  } else {
    mark = el.specific_info.Blitz.points.toString();
  }

  return {
    position: index + 1,
    mark: mark,
    its_last_played: index == info.last_in_top_five,
  };
});
</script>
