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

export function getHardDropCode(): string {
  return check(hardDropCode, hardDropCodeKey, hardDropCodeDefault);
}
export function getSoftDropCode(): string {
  return check(softDropCode, softDropCodeKey, softDropCodeDefault);
}
export function getLeftMoveCode(): string {
  return check(leftMoveCode, leftMoveCodeKey, leftMoveCodeDefault);
}
export function getRightMoveCode(): string {
  return check(rightMoveCode, rightMoveCodeKey, rightMoveCodeDefault);
}
export function getSavePieceCode(): string {
  return check(savePieceCode, savePieceCodeKey, savePieceCodeDefault);
}
export function getCounterClockwiseCode(): string {
  return check(counterClockwiseCode, counterClockwiseCodeKey, counterClockwiseCodeDefault);
}
export function getClockwiseCode(): string {
  return check(clockwiseCode, clockwiseCodeKey, clockwiseCodeDefault);
}
export function getFullRotationCode(): string {
  return check(fullRotationCode, fullRotationCodeKey, fullRotationCodeDefault);
}
export function getTargetingRandomCode(): string {
  return check(targetingRandomCode, randomCodeKey, randomCodeDefault);
}
export function getTargetingEvenCode(): string {
  return check(targetingEvenCode, evenCodeKey, evenCodeDefault);
}
export function getTargetingEliminationsCode(): string {
  return check(targetingEliminationsCode, eliminationCodeKey, eliminationCodeDefault);
}
export function getTargetingPaybackCode(): string {
  return check(targetingPaybackCode, paybackCodeKey, paybackCodeDefault);
}
export function getForfeitCode(): string {
  return check(forfeitCode, forfeitCodeKey, forfeitCodeDefault);
}
export function getRetryCode(): string {
  return check(retryCode, retryCodeKey, retryCodeDefault);
}

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

function setKeyCode(keycodeKey: string, keycodeValue: string) {
  localStorage.setItem(keycodeKey, keycodeValue);
}
function getKeyCode(keycode: string): string | null {
  return localStorage.getItem(keycode);
}