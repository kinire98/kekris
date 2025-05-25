<template>
  <div id="root">
    <div id="holder">
      <h1>
        {{ $t("ui.stats.results") }}
      </h1>
      <h2 v-if="isLines(info.last_game_info.specific_info)">
        {{ $t("ui.stats.time") }}
      </h2>
      <h2 v-else>
        {{ $t("ui.stats.points") }}
      </h2>
      <div id="mark">
        <div id="cur-mark">
          <h2>{{ valueToDisplay }}</h2>
          <div id="mark-message">
            <span v-if="info.last_in_top_five == 0">
              {{ $t("ui.stats.top_one") }}
            </span>
            <span
              v-else-if="
                info.last_in_top_five > 1 && info.last_in_top_five <= 5
              "
            >
              <h3>
                {{ $t("ui.stats.top") + " " + (info.last_in_top_five + 1) }}
              </h3>
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
        </div>
        <Divider layout="vertical" />
        <div id="personal-bests">
          <TimeMark
            v-for="el in timeMarksInfo"
            :mark="el.mark"
            :position="el.position"
            :current="el.its_last_played"
          />
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
      <div id="stats">
        <h2>{{ $t("ui.stats.stats") }}</h2>
        <div id="desglosed-stats">
          <div class="real-stats">
            <Stat
              :stat-name="$t('ui.stats.piece_moves')"
              :stat-value="info.last_game_info.piece_moves.toString()"
            />
            <Stat
              :stat-name="$t('ui.stats.pieces_used')"
              :stat-value="info.last_game_info.pieces_used.toString()"
            />
            <Stat
              :stat-name="$t('ui.stats.lines_cleared')"
              :stat-value="info.last_game_info.lines_cleared.toString()"
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
          <Divider layout="vertical" />
          <div class="real-stats">
            <Stat
              :stat-name="$t('ui.stats.piece_moves_per_second')"
              :stat-value="getXStatPerSecond(info.last_game_info.piece_moves)"
            />

            <Stat
              :stat-name="$t('ui.stats.pieces_per_second')"
              :stat-value="getXStatPerSecond(info.last_game_info.pieces_used)"
            />
            <Stat
              :stat-name="$t('ui.stats.lines_cleared_per_second')"
              :stat-value="getXStatPerSecond(info.last_game_info.lines_cleared)"
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
#holder {
  margin-block: 25px;
  scroll-behavior: smooth;
  padding-inline: 100px;
  padding-block: 35px;
  border-radius: 15px;
  background-color: #30303066;
  overflow-y: scroll;
  > h1 {
    margin: 55px;
  }
}
#cur-mark {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  > h2 {
    background-color: #30303066;
    border-radius: 15px;
    width: 100%;
    text-align: center;
    padding: 15px;
    transition: all 0.3s;
  }
  > h2:hover {
    background-color: #303030;
    transform: scale(1.1);
  }
}
#mark {
  width: 50vw;
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  flex-direction: row;
  padding: 25px;
  margin-block: 5px;
  border-radius: 15px;
  background-color: #30303066;
  > div:nth-child(2) {
    height: 250px !important;
  }
}
#mark-message {
  margin-block: 15px;
  background-color: #30303066;
  border-radius: 15px;
  width: 100%;
  text-align: left;
  line-height: 35px;
  padding-inline: 15px;
  padding-bottom: 15px;
  > h3 {
    text-align: center;
    line-height: 15px;
    width: 100%;
  }
  transition: all 0.3s;
}
#mark-message:hover {
  transform: scale(1.1);
  background-color: #303030;
}
#stats {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  width: 50vw;
  > h2 {
    width: 100%;
    text-align: left;
  }
}
#desglosed-stats {
  width: 50vw;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
  background-color: #30303066;
  padding: 15px;
  border-radius: 15px;
  > div:nth-child(2) {
    margin-inline: 35px;
    height: 550px !important;
  }
}

.real-stats {
  display: flex;
  flex-direction: column;
  width: 20vw;
}
#buttons {
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  width: 100%;
  margin-top: 50px;
  margin-bottom: 25px;
}
.p-button-outlined {
  border: 2.5px solid var(--main-color) !important;
  color: var(--main-color) !important;
  margin-block: 10px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: start;
  cursor: pointer;
}
.p-button-outlined:hover {
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
  transform: scale(1.2);
}
.p-divider-vertical {
  z-index: 98;
}
</style>
<style>
.p-button-label {
  font-family: 25px;
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

import { formatSecondsToHHMMSS } from "../helpers/formatSeconds";

useI18n();
let info: EmitGameInfo = await invoke("retreive_game_info");
let valueToDisplay: string;
let specific_info = info.last_game_info.specific_info;
let secondsPlayed: number;
let returnUrl: string;
let points: boolean = true;
if (isClassic(specific_info)) {
  valueToDisplay = specific_info.Classic.points.toString();
  secondsPlayed = specific_info.Classic.time_endured;
  returnUrl = "/classic";
} else if (isLines(specific_info)) {
  valueToDisplay = formatSecondsToHHMMSS(specific_info.Lines.time_endured);
  secondsPlayed = specific_info.Lines.time_endured;
  returnUrl = "/lines";
  points = false;
} else {
  valueToDisplay = specific_info.Blitz.points.toString();
  secondsPlayed = 120;
  returnUrl = "/blitz";
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
function getXStatPerSecond(stat: number): string {
  return Number((stat / secondsPlayed).toFixed(2)).toString() + "/S";
}
</script>
