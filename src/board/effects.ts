
const pixelsOffset = 5;
export function hardDropEffect() {
  const $board = document.getElementById("board")! as HTMLElement;
  $board.style.transform = `translateY(${pixelsOffset}px)`;
  setTimeout(() => {
    $board.style.transform = `translateY(0px)`;
  }, 100);
}
