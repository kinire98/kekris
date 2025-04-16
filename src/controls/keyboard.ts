import { invoke } from "@tauri-apps/api/core";
import { getRepeatInterval, getStartRepeatInterval } from "./interval";
import { hardDropEffect } from "../board/effects";
import { getClockwiseCode, getCounterClockwiseCode, getForfeitCode, getFullRotationCode, getHardDropCode, getLeftMoveCode, getRetryCode, getRightMoveCode, getSavePieceCode, getSoftDropCode, getTargetingEliminationsCode, getTargetingEvenCode, getTargetingPaybackCode, getTargetingRandomCode } from "./keycodes";
import { currentGameOptions } from "../board/board";

const customRepeatInterval = getRepeatInterval(); // Customize this value (in milliseconds)
const customStartRepeatInteval = getStartRepeatInterval();
let keyIntervals: Record<string, NodeJS.Timeout> = {}; // Tracks active intervals for keys
const keySet = new Set<string>();
const pressedSet = new Set<string>();
export default function manageInputListeners() {
  keyIntervals = {};
  keySet.clear();
  pressedSet.clear();
  // Handle keydown event
  document.addEventListener("keydown", keyDown);

  document.addEventListener("keyup", keyUp);
}

export function removeInputListeners() {
  document.removeEventListener("keyup", keyUp);
  document.removeEventListener("keydown", keyDown);
  for (const interval in keyIntervals) {
    if (Object.prototype.hasOwnProperty.call(keyIntervals, interval)) {
      const element = keyIntervals[interval];
      clearInterval(element);
      delete keyIntervals[interval];
    }
  }
}
function keyUp(event: KeyboardEvent) {
  if (pressedSet.has(event.key)) pressedSet.delete(event.key);
  if (event.key != getLeftMoveCode() && event.key != getRightMoveCode() && event.key != getSoftDropCode()) return;
  if (keySet.has(event.key)) keySet.delete(event.key);
  if (keyIntervals[event.key]) {
    clearInterval(keyIntervals[event.key]);
    delete keyIntervals[event.key];
  }
}
function keyDown(event: KeyboardEvent) {
  if (keyIntervals[event.key]) return;
  if (pressedSet.has(event.key)) return;
  manageInput(event.key);
  if (event.key != getLeftMoveCode() && event.key != getRightMoveCode() && event.key != getSoftDropCode()) {
    pressedSet.add(event.key);
    return;
  }
  // // If right is pressed it doesn't allow the opposite, and viceversa
  // if (event.key == getLeftMoveCode() && keySet.has(getRightMoveCode())) {
  //   return;
  // }
  // // If left is pressed it doesn't allow the opposite, and viceversa
  // if (event.key == getRightMoveCode() && keySet.has(getLeftMoveCode())) {
  //   return;
  // }
  keySet.add(event.key);
  setTimeout(() => {
    if (keySet.has(event.key)) {
      keyIntervals[event.key] = setInterval(() => {
        manageInput(event.key);
      }, customRepeatInterval);
    }
  }, customStartRepeatInteval);
}


function manageInput(keyCode: string) {
  switch (keyCode) {
    case getHardDropCode():
      hardDrop();
      break;
    case getLeftMoveCode():
      leftMove();
      break;
    case getRightMoveCode():
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
  removeInputListeners();
}

async function fullRotation() {
  await invoke("full_rotation");
}

async function hardDrop() {
  await invoke("hard_drop");
  hardDropEffect();
}

async function leftMove() {
  console.log("right");
  await invoke("left_move");
}

async function retryGame() {
  await invoke("retry_game", { options: currentGameOptions });
  const $canvas = document.getElementById("next")! as HTMLCanvasElement;
  $canvas.getContext("2d")?.clearRect(0, 0, $canvas.width, $canvas.height);
  removeInputListeners();
  setTimeout(() => {
    manageInputListeners();
  }, 3000);

}

async function rightMove() {
  console.log("right");
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
