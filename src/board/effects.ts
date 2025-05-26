import { ClearLinePattern } from "../types/ClearLinePattern";
import i18n from "../i18n";

const pixelsOffset = 5;
/**
 * Applies a hard drop effect to the game board.
 */
export function hardDropEffect() {
  const $board = document.getElementById("board")! as HTMLElement;
  $board.style.transform = `translateY(${pixelsOffset}px)`;
  setTimeout(() => {
    $board.style.transform = `translateY(0px)`;
  }, 100);
}

/**
 * Applies a lost effect to the game board.
 */
export async function lostEffect() {
  const $bgc = document.getElementById("bgc")! as HTMLElement;
  const $board = document.getElementById("board")! as HTMLElement;
  $bgc.style.backgroundImage = `radial-gradient(transparent, #ff000066)`;
  $board.classList.add("drop");
  setTimeout(() => {
    $bgc.style.backgroundImage = `radial-gradient(transparent, var(--transparent-main-color))`;
  }, 1500);
}

/**
 * Applies a line cleared effect to the game board.
 * @param pattern The clear line pattern.
 */
export function lineClearedEffect(pattern: ClearLinePattern) {
  const $el = document.getElementById("pattern")! as HTMLElement;
  $el.classList.remove("animation-letters");
  $el.innerHTML = "";
  let t = i18n.global.t;
  let value = t('board.' + pattern);

  $el.innerHTML = value;
  setTimeout(() => {
    $el.classList.add("animation-letters");
  }, 10);
  setTimeout(() => {
    $el.classList.remove("animation-letters");
    $el.innerHTML = "";
  }, 1000);
}

/**
 * Applies a piece fixed effect.
 */
export function pieceFixedEffect() {

}
