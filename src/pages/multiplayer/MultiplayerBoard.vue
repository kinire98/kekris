<template>
  <div id="wrap">
    <div id="board">
      <div id="timer">
        <div id="random" class="strategies active">
          1. {{ $t("board.strategies.random") }}
        </div>
        <div id="even" class="strategies">
          2. {{ $t("board.strategies.even") }}
        </div>
        <div id="elimination" class="strategies">
          3. {{ $t("board.strategies.elimination") }}
        </div>
        <div id="payback" class="strategies">
          4. {{ $t("board.strategies.payback") }}
        </div>
      </div>
      <div id="visible">
        <div id="left">
          <canvas height="100" width="150" id="held" class="bgc top"></canvas>
          <div id="left-bottom">
            <p id="pattern"></p>
            <div id="lines-div">
              <p>
                {{
                  $route.path.substring(1) === "blitz"
                    ? $t("board.points")
                    : $t("board.lines")
                }}
              </p>
              <p id="write-lines">0/5</p>
            </div>
          </div>
        </div>
        <canvas height="760" width="380" id="main" class="bgc"></canvas>
        <canvas height="480" width="150" id="next" class="bgc top"></canvas>
      </div>
      <canvas height="760" width="380" id="buffer"></canvas>
    </div>
    <div id="multiplayer-boards">
      <div id="left-boards"></div>
      <div id="right-boards"></div>
    </div>
  </div>
</template>
<style scoped>
#wrap {
  transition: all 3s;
}
.bgc {
  border-inline: 1px solid #909090;
  border-bottom: 1px solid #909090;
  background-color: var(--black-constrast-transparent);
}
.top {
  border-top: 1px solid #909090;
}
#board {
  position: absolute;
  left: 0;
  right: 0;
  top: -380px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column-reverse;
  height: 100%;
  transition: transform 0.1s;
}
#visible {
  display: flex;
  align-items: start;
  justify-content: center;
  flex-direction: row;
}
#left {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-direction: column;
  height: 100%;
}
#left-bottom {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-direction: column;
  height: 50%;
  opacity: 1;
  letter-spacing: 1px;
  transition: all 1s;
  font-size: 24px;
}
#pattern {
  width: 150px;
}
.animation-letters {
  opacity: 0;
  letter-spacing: 5px;
  transform: translateY(-15px);
  transition: all 2s;
}
#multiplayer-boards {
  position: absolute;
  top: 2vh;
  bottom: 2vh;
  left: 2vw;
  right: 2vw;
  display: flex;
  align-items: center;
  justify-content: space-between;
}
#left-boards,
#right-boards {
  width: 25%;
  height: 100%;
  display: grid;
  grid-template-columns: 50% 50%;
  grid-template-rows: 25% 25% 25% 25%;
  align-items: center;
  justify-content: center;
  place-items: center;
}
canvas.multiplayer {
  border-inline: 1px solid #909090;
  border-bottom: 1px solid #909090;
}
.drop {
  transform: rotate(-15deg);
  animation: drop_board 1.5s ease-in forwards;
}
@keyframes drop_board {
  0% {
    transform: translateY(0px) rotate(-15deg);
    opacity: 1;
    visibility: visible;
  }

  10% {
    transform: translateY(2px) rotate(-15deg);
    opacity: 0.998046875;
  }

  20% {
    transform: translateY(4px) rotate(-15deg);
    opacity: 0.99609375;
  }

  30% {
    transform: translateY(8px) rotate(-15deg);
    opacity: 0.9921875;
  }

  40% {
    transform: translateY(16px) rotate(-15deg);
    opacity: 0.984375;
  }

  50% {
    transform: translateY(32px) rotate(-15deg);
    opacity: 0.96875;
  }

  60% {
    transform: translateY(64px) rotate(-15deg);
    opacity: 0.9375;
  }

  70% {
    transform: translateY(128px) rotate(-15deg);
    opacity: 0.875;
  }

  80% {
    transform: translateY(256px) rotate(-15deg);
    opacity: 0.75;
  }

  90% {
    transform: translateY(512px) rotate(-15deg);
    opacity: 0.5;
  }

  100% {
    transform: translateY(1024px) rotate(-15deg);
    opacity: 0;
    visibility: hidden;
  }
}
.won {
  position: relative;
  opacity: 0;
  top: 50vh;
  transform: scale(2);
}
#timer {
  margin-top: 15px;
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  flex-direction: row;
}
.strategies {
  border-radius: 15px;
  color: #909090;
  border: 1px solid #909090;
  width: 150px;
  text-align: center;
}
.strategies.active {
  color: var(--main-contrast);
  background: #909090;
}
.canvases {
  transition: opacity 1.5s;
}
.canvases.drop {
  transform: rotate(-15deg);
  animation: drop_board 1.5s ease-in forwards;
}
</style>
<script lang="ts" setup>
import startDraw from "../../board/board";
import manageInput from "../../controls/keyboard";
import startHeld from "../../board/held_piece";
import startQueue from "../../board/queue";
import { GameOptions } from "../../types/GameOptions";
import { useI18n } from "vue-i18n";
import { onMounted } from "vue";
import { useRoute } from "vue-router";
useI18n();
let modifier = 1;
const route = useRoute();

const canvasWidth = 109;
const canvasHeight = 217;
onMounted(() => {
  const leftBoards = document.getElementById("left-boards")!;
  const rightBoards = document.getElementById("right-boards")!;
  let players = parseInt(route.params.players! as string);
  let id = parseInt(route.params.id! as string);
  for (let i = 0 + modifier; i < players + modifier; i++) {
    if (i == id) {
      continue;
    }
    const canvas = document.createElement("canvas");
    canvas.id = "board-player-" + i;
    canvas.height = canvasHeight;
    canvas.width = canvasWidth;
    canvas.classList.add("canvases");
    if (i < 8 + modifier) {
      leftBoards.appendChild(canvas);
    } else {
      rightBoards.appendChild(canvas);
    }
    canvas.style.borderInline = "1px solid #909090";
    canvas.style.borderBottom = "1px solid #909090";
  }
  let options: GameOptions = {
    number_of_players: players,
    lines_40: false,
    blitz: false,
    normal: true,
  };
  startDraw(
    document.getElementById("main")! as HTMLCanvasElement,
    document.getElementById("buffer")! as HTMLCanvasElement,
    options
  );
  startHeld(document.getElementById("held")! as HTMLCanvasElement);
  startQueue(document.getElementById("next")! as HTMLCanvasElement);
  setTimeout(() => {
    manageInput();
  }, 3001);
});
</script>
