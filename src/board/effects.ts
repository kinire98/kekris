import { ClearLinePattern } from "../types/ClearLinePattern";
import i18n from "../i18n";

const pixelsOffset = 5;
export function hardDropEffect() {
  const $board = document.getElementById("board")! as HTMLElement;
  $board.style.transform = `translateY(${pixelsOffset}px)`;
  setTimeout(() => {
    $board.style.transform = `translateY(0px)`;
  }, 100);
}

export async function lostEffect() {
  const $bgc = document.getElementById("bgc")! as HTMLElement;
  const $board = document.getElementById("board")! as HTMLElement;
  $bgc.style.backgroundImage = `radial-gradient(transparent, #ff000066)`;
  $board.classList.add("drop");
  setTimeout(() => {
    $bgc.style.backgroundImage = `radial-gradient(transparent, var(--transparent-main-color))`;
  }, 1500);
}

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

export function pieceFixedEffect() {

}

export function gameWonEffect() {
  const $board = document.getElementById("wrap")! as HTMLElement;
  $board.classList.add("won");

}

