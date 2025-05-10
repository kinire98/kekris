<template>
  <div id="wrap">
    <div id="board">
      <div id="timer" v-if="$route.path.substring(1) !== 'classic'">
        00:00:00
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
              <p id="write-lines">
                {{
                  $route.path.substring(1) === "classic"
                    ? "0/5"
                    : $route.path.substring(1) === "lines"
                    ? "0/40"
                    : "0"
                }}
              </p>
            </div>
          </div>
        </div>
        <canvas height="760" width="380" id="main" class="bgc"></canvas>
        <canvas height="480" width="150" id="next" class="bgc top"></canvas>
      </div>
      <canvas height="760" width="380" id="buffer"></canvas>
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

.drop {
  transform: rotate(-15deg);
  animation: drop_board 1.5s ease-in forwards;
}
@keyframes drop_board {
  0% {
    transform: translateY(0px) rotate(-15deg);
  }

  10% {
    transform: translateY(2px) rotate(-15deg);
  }

  20% {
    transform: translateY(4px) rotate(-15deg);
  }

  30% {
    transform: translateY(8px) rotate(-15deg);
  }

  40% {
    transform: translateY(16px) rotate(-15deg);
  }

  50% {
    transform: translateY(32px) rotate(-15deg);
  }

  60% {
    transform: translateY(64px) rotate(-15deg);
  }

  70% {
    transform: translateY(128px) rotate(-15deg);
  }

  80% {
    transform: translateY(256px) rotate(-15deg);
  }

  90% {
    transform: translateY(512px) rotate(-15deg);
  }

  100% {
    transform: translateY(1024px) rotate(-15deg);
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
}
</style>
<script lang="ts">
import startDraw from "../../board/board";
import manageInput from "../../controls/keyboard";
import startHeld from "../../board/held_piece";
import startQueue from "../../board/queue";
import { GameOptions } from "../../types/GameOptions";
import { useI18n } from "vue-i18n";
export default {
  mounted() {
    let name = this.$route.path;
    let options: GameOptions;
    name = name.substring(1);
    switch (name) {
      case "classic":
        options = {
          number_of_players: 1,
          lines_40: false,
          normal: true,
          blitz: false,
        };
        break;
      case "lines":
        options = {
          number_of_players: 1,
          lines_40: true,
          normal: false,
          blitz: false,
        };
        break;
      case "blitz":
        options = {
          number_of_players: 1,
          lines_40: false,
          normal: false,
          blitz: true,
        };
        break;
      default:
        options = {
          number_of_players: 1,
          lines_40: false,
          normal: true,
          blitz: false,
        };
        break;
    }
    // startDraw(
    //   document.getElementById("main")! as HTMLCanvasElement,
    //   document.getElementById("buffer")! as HTMLCanvasElement,
    //   options
    // );
    // startHeld(document.getElementById("held")! as HTMLCanvasElement);
    // startQueue(document.getElementById("next")! as HTMLCanvasElement);
    // setTimeout(() => {
    //   manageInput();
    // }, 3001);
  },
  setup() {
    const t = useI18n();
    return t;
  },
};
</script>
