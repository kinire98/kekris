import { invoke } from "@tauri-apps/api/core";
import { getRepeatInterval, getStartRepeatInterval } from "./interval";
import { hardDropEffect } from "../board/effects";
import { getClockwiseCode, getCounterClockwiseCode, getForfeitCode, getFullRotationCode, getHardDropCode, getLeftMoveCode, getRetryCode, getRightMoveCode, getSavePieceCode, getSoftDropCode, getTargetingEliminationsCode, getTargetingEvenCode, getTargetingPaybackCode, getTargetingRandomCode } from "./keycodes";
import { currentGameOptions } from "../board/board";

let customRepeatInterval = getRepeatInterval();
let customStartRepeatInterval = getStartRepeatInterval();
let keyIntervals: Record<string, NodeJS.Timeout> = {};
const keySet = new Set<string>(); // Repeat interval
const pressedSet = new Set<string>(); // Not repeat

/**
 * Manages the keyboard input listeners for the game.
 *
 * This function sets up the keydown and keyup event listeners to handle user input.
 * It also initializes the keyIntervals, keySet, and pressedSet variables.
 */
export default function manageInputListeners() {
  keyIntervals = {};
  keySet.clear();
  pressedSet.clear();
  customRepeatInterval = getRepeatInterval();
  customStartRepeatInterval = getStartRepeatInterval();
  // Handle keydown event
  document.addEventListener("keydown", keyDown);

  document.addEventListener("keyup", keyUp);
}

/**
 * Removes the keyboard input listeners.
 *
 * This function removes the keydown and keyup event listeners to stop handling user input.
 */
export function removeInputListeners() {
  document.removeEventListener("keyup", keyUp);
  document.removeEventListener("keydown", keyDown);
  removeIntervals();
}
/**
 * Handles the keyup event.
 *
 * This function is called when a key is released. It removes the key from the pressedSet and keySet,
 * and clears any intervals associated with the key.
 * @param event The keyboard event.
 */
function keyUp(event: KeyboardEvent) {
  if (pressedSet.has(event.key)) {
    pressedSet.delete(event.key);
    return;
  }

  if (event.key != getLeftMoveCode() && event.key != getRightMoveCode() && event.key != getSoftDropCode()) {
    return;
  }

  if (keySet.has(event.key)) {
    keySet.delete(event.key);
    removeIntervals();
  }
}
/**
 * Handles the keydown event.
 *
 * This function is called when a key is pressed. It adds the key to the keySet and pressedSet,
 * and sets up an interval to repeatedly call the manageInput function while the key is held down.
 * @param event The keyboard event.
 */
function keyDown(event: KeyboardEvent) {
  if (keySet.has(event.key)) {
    return;
  }
  if (pressedSet.has(event.key)) {
    return;
  }

  manageInput(event.key);
  if (event.key == getHardDropCode()) {
    removeIntervals();
  }
  if (event.key != getLeftMoveCode() && event.key != getRightMoveCode() && event.key != getSoftDropCode()) {
    pressedSet.add(event.key);
    return;
  }


  keySet.add(event.key);


  setTimeout(() => {
    if (keySet.has(event.key) && !keyIntervals.hasOwnProperty(event.key)) {
      keyIntervals[event.key] = setInterval(() => {
        manageInput(event.key);
      }, customRepeatInterval);
    }
  }, customStartRepeatInterval);
}

/**
 * Removes all active intervals.
 */
function removeIntervals() {
  for (const interval in keyIntervals) {
    if (Object.prototype.hasOwnProperty.call(keyIntervals, interval)) {
      const element = keyIntervals[interval];
      clearInterval(element);
      delete keyIntervals[interval];
    }
  }
}

/**
 * Manages the input based on the key code.
 * @param keyCode The key code of the key that was pressed.
 */
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
  await invoke("left_move");
}

async function retryGame() {
  if (currentGameOptions.number_of_players > 1) {
    return;
  }
  await invoke("retry_game", { options: currentGameOptions });
  const $canvas = document.getElementById("next")! as HTMLCanvasElement;
  $canvas.getContext("2d")?.clearRect(0, 0, $canvas.width, $canvas.height);
  removeInputListeners();
  setTimeout(() => {
    manageInputListeners();
  }, 3000);

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
  changeStrategy(2);
}

async function targetingEven() {
  await invoke("targeting_strategy_even");
  changeStrategy(1);
}

async function targetingRandom() {
  await invoke("targeting_strategy_random");
  changeStrategy(0);
}

async function targetingPayback() {
  await invoke("targeting_strategy_payback");
  changeStrategy(3);
}

function changeStrategy(position: number) {
  let strategies = document.querySelectorAll(".strategies");
  if (strategies.length == 0) {
    return;
  }
  strategies.forEach(el => el.classList.remove("active"));
  strategies[position].classList.add("active");
}


