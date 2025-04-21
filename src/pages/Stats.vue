<template>
  <div id="root">
    <h1 v-if="isLines(info.last_game_info.specific_info)">
      {{ $t("ui.stats.time") }}
    </h1>
    <h1 v-else>
      {{ $t("ui.stats.points") }}
    </h1>
    <div id="mark">
      <div id="cur-mark">{{ valueToDisplay }}</div>
      <div id="personal-bests">
        <TimeMark
          v-for="el in timeMarksInfo"
          :mark="el.mark"
          :position="el.position"
          :current="el.its_last_played"
        />
      </div>
    </div>
    <div id="mark-message">
      <span v-if="info.last_in_top_five == 1">
        {{ $t("ui.stats.top_one") }}
      </span>
      <span v-if="info.last_in_top_five > 1 && info.last_in_top_five <= 5">
        {{ $t("ui.stats.top") + " " + (info.last_in_top_five + 1) }} <br />
        {{
          behindFirst() +
          (points ? " " + $t("ui.stats.points_lower") + " " : " ") +
          $t("ui.stats.behind_1st")
        }}
      </span>
      <span v-else>
        {{
          behindFirst() +
          (points ? " " + $t("ui.stats.points_lower") + " " : " ") +
          $t("ui.stats.behind_1st")
        }}
      </span>
    </div>
    <div id="stats">
      <h2>{{ $t("ui.stats.stats") }}</h2>
      <div id="desglosed-stats">
        <div>
          <Stat
            :stat-name="$t('ui.stats.piece_moves')"
            :stat-value="info.last_game_info.piece_moves.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.spins')"
            :stat-value="info.last_game_info.spins.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.singles')"
            :stat-value="info.last_game_info.singles.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.triples')"
            :stat-value="info.last_game_info.triples.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.tspins')"
            :stat-value="info.last_game_info.tspins.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.tspin_doubles')"
            :stat-value="info.last_game_info.tspin_doubles.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.minitspins')"
            :stat-value="info.last_game_info.minitspins.toString()"
          />
        </div>
        <div>
          <Stat
            :stat-name="$t('ui.stats.pieces_used')"
            :stat-value="info.last_game_info.pieces_used.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.pieces_per_second')"
            :stat-value="
              Number(
                (info.last_game_info.pieces_used / secondsPlayed).toFixed(2)
              ).toString() + '/S'
            "
          />
          <Stat
            :stat-name="$t('ui.stats.doubles')"
            :stat-value="info.last_game_info.doubles.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.tetrises')"
            :stat-value="info.last_game_info.tetrises.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.tspin_singles')"
            :stat-value="info.last_game_info.tspin_singles.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.tspin_triples')"
            :stat-value="info.last_game_info.tspin_triples.toString()"
          />
          <Stat
            :stat-name="$t('ui.stats.minitspins_singles')"
            :stat-value="info.last_game_info.minitspin_singles.toString()"
          />
        </div>
      </div>
    </div>
    <div id="buttons">
      <Button
        :label="$t('ui.stats.main_menu')"
        id="back"
        variant="outlined"
        raised
        @click="mainMenu"
      />
      <Button
        :label="$t('ui.stats.play_again')"
        id="back"
        variant="outlined"
        raised
        @click="playAgain"
      />
    </div>
  </div>
</template>
<style scoped>
#root {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  /* z-index: 5; */
  height: 100vh;
  width: 100vw;
}
#mark {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
}
#stats {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
}
#desglosed-stats {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
  div {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
  }
}
#buttons {
  display: flex;
  align-items: center;
  justify-content: center;
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
</style>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import {
  EmitGameInfo,
  isBlitz,
  isClassic,
  isLines,
} from "../types/EmitGameInfo";
import { useI18n } from "vue-i18n";
import TimeMark from "../components/TimeMark.vue";
import { TimeMarkInfo } from "../types/TimeMarkInfo";
import Stat from "../components/Stat.vue";
import { Button } from "primevue";
import { router } from "../router";

let t = useI18n();
let info: EmitGameInfo = await invoke("retreive_game_info");
let valueToDisplay: string;
let specific_info = info.last_game_info.specific_info;
let secondsPlayed;
let returnUrl: string;
let points: boolean = false;
if (isClassic(specific_info)) {
  valueToDisplay = specific_info.Classic.points.toString();
  secondsPlayed = specific_info.Classic.time_endured;
  returnUrl = "/classic";
} else if (isLines(specific_info)) {
  valueToDisplay = formatSecondsToHHMMSS(specific_info.Lines.time_endured);
  secondsPlayed = specific_info.Lines.time_endured;
  returnUrl = "/lines";
} else {
  valueToDisplay = specific_info.Blitz.points.toString();
  secondsPlayed = 120;
  returnUrl = "/blitz";
  points = true;
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

function playAgain() {
  router.push(returnUrl);
}
function mainMenu() {
  router.push("/main");
}

function formatSecondsToHHMMSS(secondsMark: number): string {
  const hours = Math.floor(secondsMark / 3600);
  const minutes = Math.floor((secondsMark % 3600) / 60);
  const seconds = secondsMark % 60;

  const pad = (n: number) => n.toString().padStart(2, "0");

  return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`;
}
function behindFirst(): string {
  if (
    isClassic(info.last_game_info.specific_info) &&
    isClassic(info.top_five_results[0].specific_info)
  ) {
    return (
      info.top_five_results[0].specific_info.Classic.points -
      info.last_game_info.specific_info.Classic.points
    ).toString();
  } else if (
    isLines(info.last_game_info.specific_info) &&
    isLines(info.top_five_results[0].specific_info)
  ) {
    return formatSecondsToHHMMSS(
      info.last_game_info.specific_info.Lines.time_endured -
        info.top_five_results[0].specific_info.Lines.time_endured
    );
  } else if (
    isBlitz(info.last_game_info.specific_info) &&
    isBlitz(info.top_five_results[0].specific_info)
  ) {
    return (
      info.top_five_results[0].specific_info.Blitz.points -
      info.last_game_info.specific_info.Blitz.points
    ).toString();
  }
  return "";
}
</script>
