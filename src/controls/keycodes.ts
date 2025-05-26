export const hardDropCodeKey = "hardDrop";
export const softDropCodeKey = "softDrop";
export const leftMoveCodeKey = "leftMove";
export const rightMoveCodeKey = "rightMove";
export const savePieceCodeKey = "savePiece";
export const counterClockwiseCodeKey = "counterClockwise";
export const clockwiseCodeKey = "clockwise";
export const fullRotationCodeKey = "full";
export const randomCodeKey = "random";
export const evenCodeKey = "even";
export const eliminationCodeKey = "eliminations";
export const paybackCodeKey = "payback";
export const forfeitCodeKey = "forfeit";
export const retryCodeKey = "retry";
const hardDropCodeDefault = "ArrowDown";
const softDropCodeDefault = " ";
const leftMoveCodeDefault = "ArrowLeft";
const rightMoveCodeDefault = "ArrowRight";
const savePieceCodeDefault = "Shift";
const counterClockwiseCodeDefault = "a";
const clockwiseCodeDefault = "d";
const fullRotationCodeDefault = "s";
const randomCodeDefault = "1";
const evenCodeDefault = "2";
const eliminationCodeDefault = "3";
const paybackCodeDefault = "4";
const forfeitCodeDefault = "Escape";
const retryCodeDefault = "p";

let hardDropCode = "";
let softDropCode = "";
let leftMoveCode = "";
let rightMoveCode = "";
let savePieceCode = "";
let counterClockwiseCode = "";
let clockwiseCode = "";
let fullRotationCode = "";
let targetingRandomCode = "";
let targetingEvenCode = "";
let targetingEliminationsCode = "";
let targetingPaybackCode = "";
let forfeitCode = "";
let retryCode = "";

/**
 * Gets the hard drop key code.
 * @returns The hard drop key code.
 */
export function getHardDropCode(): string {
  return check(hardDropCode, hardDropCodeKey, hardDropCodeDefault);
}
/**
 * Gets the soft drop key code.
 * @returns The soft drop key code.
 */
export function getSoftDropCode(): string {
  return check(softDropCode, softDropCodeKey, softDropCodeDefault);
}
/**
 * Gets the left move key code.
 * @returns The left move key code.
 */
export function getLeftMoveCode(): string {
  return check(leftMoveCode, leftMoveCodeKey, leftMoveCodeDefault);
}
/**
 * Gets the right move key code.
 * @returns The right move key code.
 */
export function getRightMoveCode(): string {
  return check(rightMoveCode, rightMoveCodeKey, rightMoveCodeDefault);
}
/**
 * Gets the save piece key code.
 * @returns The save piece key code.
 */
export function getSavePieceCode(): string {
  return check(savePieceCode, savePieceCodeKey, savePieceCodeDefault);
}
/**
 * Gets the counter-clockwise rotation key code.
 * @returns The counter-clockwise rotation key code.
 */
export function getCounterClockwiseCode(): string {
  return check(counterClockwiseCode, counterClockwiseCodeKey, counterClockwiseCodeDefault);
}
/**
 * Gets the clockwise rotation key code.
 * @returns The clockwise rotation key code.
 */
export function getClockwiseCode(): string {
  return check(clockwiseCode, clockwiseCodeKey, clockwiseCodeDefault);
}
/**
 * Gets the full rotation key code.
 * @returns The full rotation key code.
 */
export function getFullRotationCode(): string {
  return check(fullRotationCode, fullRotationCodeKey, fullRotationCodeDefault);
}
/**
 * Gets the targeting random key code.
 * @returns The targeting random key code.
 */
export function getTargetingRandomCode(): string {
  return check(targetingRandomCode, randomCodeKey, randomCodeDefault);
}
/**
 * Gets the targeting even key code.
 * @returns The targeting even key code.
 */
export function getTargetingEvenCode(): string {
  return check(targetingEvenCode, evenCodeKey, evenCodeDefault);
}
/**
 * Gets the targeting eliminations key code.
 * @returns The targeting eliminations key code.
 */
export function getTargetingEliminationsCode(): string {
  return check(targetingEliminationsCode, eliminationCodeKey, eliminationCodeDefault);
}
/**
 * Gets the targeting payback key code.
 * @returns The targeting payback key code.
 */
export function getTargetingPaybackCode(): string {
  return check(targetingPaybackCode, paybackCodeKey, paybackCodeDefault);
}
/**
 * Gets the forfeit action key code.
 * @returns The forfeit action key code.
 */
export function getForfeitCode(): string {
  return check(forfeitCode, forfeitCodeKey, forfeitCodeDefault);
}
/**
 * Gets the retry action key code.
 * @returns The retry action key code.
 */
export function getRetryCode(): string {
  return check(retryCode, retryCodeKey, retryCodeDefault);
}

/**
 * Checks if the code is valid and returns it.
 * If the code is not valid, it gets the code from local storage.
 * If the code is not in local storage, it sets the code to the default value and returns the default value.
 * @param code The code to check.
 * @param key The key to use for local storage.
 * @param defaultValue The default value to use if the code is not valid.
 * @returns The valid code.
 */
function check(code: string, key: string, defaultValue: string): string {
  if (code.length !== 0) {
    return code;
  }
  let valueFromPreferences = getKeyCode(key);
  if (valueFromPreferences != null) {
    code = valueFromPreferences;
    return code;
  }
  setKeyCode(key, defaultValue);
  return defaultValue;
}

/**
 * Sets the key code in local storage.
 * @param keycodeKey The key to use for local storage.
 * @param keycodeValue The value to store in local storage.
 */
function setKeyCode(keycodeKey: string, keycodeValue: string) {
  localStorage.setItem(keycodeKey, keycodeValue);
}
/**
 * Gets the key code from local storage.
 * @param keycode The key to use for local storage.
 * @returns The key code from local storage.
 */
function getKeyCode(keycode: string): string | null {
  return localStorage.getItem(keycode);
}