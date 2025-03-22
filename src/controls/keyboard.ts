import { invoke } from "@tauri-apps/api/core";
import { getRepeatInterval, getStartRepeatInterval } from "./interval";
import { hardDropEffect } from "../board/effects";
import { getClockwiseCode, getCounterClockwiseCode, getForfeitCode, getFullRotationCode, getHardDropCode, getLeftMoveCode, getPauseCode, getRetryCode, getRigthMoveCode, getSavePieceCode, getSoftDropCode, getTargetingEliminationsCode, getTargetingEvenCode, getTargetingPaybackCode, getTargetingRandomCode } from "./keycodes";

export default function manageInputListeners() {
  // ! Take out to another file
  const customRepeatInterval = getRepeatInterval(); // Customize this value (in milliseconds)
  const customStartRepeatInteval = getStartRepeatInterval();
  const keyIntervals: Record<string, NodeJS.Timeout> = {}; // Tracks active intervals for keys
  const keySet = new Set<string>();
  const pressedSet = new Set<string>();

  // Handle keydown event
  document.addEventListener("keydown", (event: KeyboardEvent) => {
    if (keyIntervals[event.key]) return;
    if (pressedSet.has(event.key)) return;
    // Trigger the action immediately
    manageInput(event.key);

    // Set up a custom interval for repeated actions

    if (event.key != getLeftMoveCode() && event.key != getRigthMoveCode() && event.key != getSoftDropCode()) {
      pressedSet.add(event.key);
      return;
    }
    keySet.add(event.key);
    setTimeout(() => {
      if (keySet.has(event.key)) {
        keyIntervals[event.key] = setInterval(() => {
          manageInput(event.key);
        }, customRepeatInterval);
      }
    }, customStartRepeatInteval);
  });

  // Handle keyup event
  document.addEventListener("keyup", (event: KeyboardEvent) => {
    if (pressedSet.has(event.key)) pressedSet.delete(event.key);
    if (event.key != getLeftMoveCode() && event.key != getRigthMoveCode() && event.key != getSoftDropCode()) return;
    if (keySet.has(event.key)) keySet.delete(event.key);
    if (keyIntervals[event.key]) {
      clearInterval(keyIntervals[event.key]);
      delete keyIntervals[event.key];
    }
  });
}


function manageInput(keyCode: string) {
  switch (keyCode) {
    case getHardDropCode():
      hardDrop();
      break;
    case getLeftMoveCode():
      leftMove();
      break;
    case getRigthMoveCode():
      rightMove();
      break;
    case getSavePieceCode():
      savePiece();
      break;
    case getSoftDropCode():
      softDrop();
      break;
    case getCounterClockwiseCode():
      counterClockWise();
      break;
    case getFullRotationCode():
      fullRotation();
      break;
    case getClockwiseCode():
      clockwise();
      break;
    case getTargetingRandomCode():
      targetingRandom();
      break;
    case getTargetingEvenCode():
      targetingEven();
      break;
    case getTargetingEliminationsCode():
      targetingEliminations();
      break;
    case getTargetingPaybackCode():
      targetingPayback();
      break;
    case getForfeitCode():
      forfeit();
      break;
    case getRetryCode():
      retryGame();
      break;
    case getPauseCode():
      pauseGame();
      break;
  }
}

async function clockwise() {
  await invoke("clockwise_rotation");
}

async function counterClockWise() {
  await invoke("counter_clockwise_rotation");
}

async function forfeit() {
  await invoke("forfeit_game")
}

async function fullRotation() {
  await invoke("full_rotation");
}

async function hardDrop() {
  await invoke("hard_drop");
  hardDropEffect();
}

async function leftMove() {
  await invoke("left_move");
}

async function retryGame() {
  await invoke("retry_game");
}

async function rightMove() {
  await invoke("right_move");
}

async function savePiece() {
  await invoke("save_piece");
}

async function softDrop() {
  await invoke("soft_drop");
}

async function targetingEliminations() {
  await invoke("targeting_strategy_eliminations");
}

async function targetingEven() {
  await invoke("targeting_strategy_even");
}

async function targetingRandom() {
  await invoke("targeting_strategy_random");
}

async function targetingPayback() {
  await invoke("targeting_strategy_payback");
}
async function pauseGame() {
  await invoke("pause_game");
}
