const hardDropCodeKey = "hardDrop";
const hardDropCodeDefault = "ArrowDown";
const softDropCodeKey = "softDrop";
const softDropCodeDefault = " ";
const leftMoveCodeKey = "leftMove";
const leftMoveCodeDefault = "ArrowLeft";
const rightMoveCodeKey = "rightMove";
const rightMoveCodeDefault = "ArrowRight";
const savePieceCodeKey = "savePiece";
const savePieceCodeDefault = "Shift";
const counterClockwiseCodeKey = "counterClockwise";
const counterClockwiseCodeDefault = "a";
const clockwiseCodeKey = "clockwise";
const clockwiseCodeDefault = "d";
const fullRotationCodeKey = "full";
const fullRotationCodeDefault = "s";
const randomCodeKey = "random";
const randomCodeDefault = "1";
const evenCodeKey = "even";
const evenCodeDefault = "2";
const eliminationCodeKey = "eliminations";
const eliminationCodeDefault = "3";
const paybackCodeKey = "payback";
const paybackCodeDefault = "4";
const forfeitCodeKey = "forfeit";
const forfeitCodeDefault = "Escape";
const retryCodeKey = "retry";
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
export function getRigthMoveCode(): string {
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