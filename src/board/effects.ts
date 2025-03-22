import type { ClearLinePattern } from "../types/ClearLinePattern";

const pixelsOffset = 5;
export function hardDropEffect() {
  const $board = document.getElementById("board")! as HTMLElement;
  $board.style.transform = `translateY(${pixelsOffset}px)`;
  setTimeout(() => {
    $board.style.transform = `translateY(0px)`;
  }, 100);
}

export function lostEffect() {
  const $bgc = document.getElementById("bgc")! as HTMLElement;
  const $board = document.getElementById("board")! as HTMLElement;
  $bgc.style.backgroundImage = `radial-gradient(transparent, #ff000066)`;
  $board.classList.add("drop");
  setTimeout(() => {
    $bgc.style.backgroundImage = `radial-gradient(transparent, #222831)`;
    const $a = document.createElement("a");
    $a.href = "/";
    $a.style.display = "none";
    document.body.append($a);
    $a.click();
  }, 1500);
}

export function lineClearedEffect(pattern: ClearLinePattern) {

}

export function pieceFixedEffect() {

}

export function gameWonEffect() {

}


