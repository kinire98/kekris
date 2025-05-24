
import { listen } from "@tauri-apps/api/event";
import {
  getBorderColor, getIPieceColor, getIPieceDarkColor, getJPieceColor,
  getJPieceDarkColor, getLPieceColor, getLPieceDarkColor, getOPieceColor,
  getOPieceDarkColor, getSPieceColor, getSPieceDarkColor, getTPieceColor,
  getTPieceDarkColor, getZPieceColor, getZPieceDarkColor,
  trashColor,
  trashBorderColor,
  getGhostColor
} from "./colors";
import { invoke } from "@tauri-apps/api/core";
import type { GameOptions } from "../types/GameOptions";
import { lineClearedEffect, lostEffect, pieceFixedEffect } from "./effects";
import type { ClearLinePattern } from "../types/ClearLinePattern";
import { removeInputListeners } from "../controls/keyboard";
import { UnlistenFn } from "@tauri-apps/api/event";
import { Piece } from "../types/Piece";
import { router } from "../router";
import { todo } from "node:test";
import { OtherPlayerState, Player, WonSignal } from "../types/Room";

const canvasHeight = 760;
const canvasWidth = 380;

const columnNumber = 10;
const rowNumber = 20;

const spacing = 38;
const pieceWidth = canvasWidth / 10;
const pieceHeight = canvasHeight / 20;


const boardSize = columnNumber * rowNumber;
const boardStateEmit = "board_state_emit";
const lineClearedEmit = "line_cleared";
const pieceFixed = "piece_fixed";
const pointsEmit = "points";
const gameOverEmit = "game_over";
const gameWonEmit = "game_won";
const lineClearedInfoEmit = "line_cleared_info";
const timeEmit = "time_emit";

const stateEmitForOtherPlayers = "stateEmitForOtherPlayers";
const otherPlayerLostEmit = "otherPlayerLostEmit";
const otherPlayerWonEmit = "otherPlayerWon";

const unlisteners: UnlistenFn[] = [];
const multiplayerUnlisteners: UnlistenFn[] = [];
// E -> Empty
// C -> Clear
// G -> Ghost
// O -> yellow
// I -> cyan
// T -> Purple
// L -> orange
// J -> blue
// S -> green
// Z -> red

let mainCanvas: HTMLCanvasElement;
let bufferCanvas: HTMLCanvasElement;

export let currentGameOptions: GameOptions;
export default function startDraw(canvas: HTMLCanvasElement, secondCanvas: HTMLCanvasElement, options: GameOptions) {
  currentGameOptions = options;
  const ctx: CanvasRenderingContext2D = canvas.getContext("2d")!;
  mainCanvas = canvas;
  bufferCanvas = secondCanvas;
  drawLines(ctx);
  startBoardChangeEventListener();
  if (options.number_of_players == 1) {
    invoke("start_game", {
      options: options
    });
  } else {
    boards();
    otherPlayerLost();
    otherPlayerWon();
  }
  gameLost();
  lineCleared();
  pieceFixedEvent();
  if (options.normal || options.lines_40) {
    lineClearedInfo();
  } else {
    pointsInfo();
  }
  if (!options.normal || options.number_of_players > 1) {
    gameWon();
  }
  if (!options.normal) {
    timer();
  }
}
function drawBufferBoard(board: string) {
  const ctx: CanvasRenderingContext2D = bufferCanvas.getContext("2d")!;
  drawBoardInternal(board, ctx, false);
}
function drawMainBoard(board: string) {
  const ctx: CanvasRenderingContext2D = mainCanvas.getContext("2d")!;
  drawBoardInternal(board, ctx, true);
}
function drawBoardInternal(board: string, ctx: CanvasRenderingContext2D, drawLDivisories: boolean) {
  clearCanvas(ctx);
  if (drawLDivisories) {
    drawLines(ctx);
  }
  for (let i = boardSize - 1; i > -1; i--) {
    const piece: Piece = board[i]! as Piece;
    if (piece == Piece.Empty)
      continue;
    const y = Math.floor(i / columnNumber);
    const x = i % columnNumber;
    if (piece == Piece.Trash) {
      trashPiece(ctx, x, y);
      continue;
    }
    if (piece == Piece.Ghost) {
      ghostPiece(ctx, x, y);
      continue;
    }
    const color = getColor(piece);
    const darkColor = getDarkColor(piece);
    ctx.strokeStyle = color;
    ctx.lineWidth = 8;
    const widthSecondRing = 5;
    ctx.strokeRect(widthSecondRing + (pieceWidth * x), widthSecondRing + (pieceWidth * y), pieceWidth - (widthSecondRing * 2), pieceHeight - (widthSecondRing * 2));
    ctx.strokeStyle = darkColor;
    ctx.lineWidth = 2;
    ctx.strokeRect(1 + (pieceWidth * x), 1 + (pieceHeight * y), pieceWidth - 1, pieceHeight - 1);
    ctx.strokeStyle = darkColor;
    ctx.lineWidth = 2;
    ctx.strokeRect(10 + (pieceWidth * x), 10 + (pieceHeight * y), pieceWidth / 2, pieceHeight / 2);

    ctx.fillStyle = color;
    ctx.fillRect(11 + (pieceWidth * x), 11 + (pieceHeight * y), pieceWidth / 3 + 4, pieceHeight / 3 + 4);
  }
}


async function startBoardChangeEventListener() {
  await listen<string>(boardStateEmit, e => {
    drawBoard(e.payload);
  })
}


function drawLines(ctx: CanvasRenderingContext2D) {
  ctx.lineWidth = 1;
  ctx.strokeStyle = getBorderColor();
  for (let i = 1; i < columnNumber; i++) {
    ctx.beginPath();
    ctx.moveTo(spacing * i, 1);
    ctx.lineTo(spacing * i, canvasHeight - 1);
    ctx.stroke();
  }
  for (let i = 1; i < rowNumber; i++) {
    ctx.beginPath();
    ctx.moveTo(1, spacing * i);
    ctx.lineTo(canvasWidth - 1, spacing * i);
    ctx.stroke();
  }
}

function getColor(piece: string): string {
  if (piece == "E")
    return "transparent";
  else if (piece == "O")
    return getOPieceColor();
  else if (piece == "I")
    return getIPieceColor();
  else if (piece == "T")
    return getTPieceColor();
  else if (piece == "L")
    return getLPieceColor();
  else if (piece == "J")
    return getJPieceColor();
  else if (piece == "S")
    return getSPieceColor();
  else if (piece == "Z")
    return getZPieceColor();
  throw new Error("Invalid Value");
}
function getDarkColor(piece: string): string {
  if (piece == "E")
    return "transparent";
  else if (piece == "O")
    return getOPieceDarkColor();
  else if (piece == "I")
    return getIPieceDarkColor();
  else if (piece == "T")
    return getTPieceDarkColor();
  else if (piece == "L")
    return getLPieceDarkColor();
  else if (piece == "J")
    return getJPieceDarkColor();
  else if (piece == "S")
    return getSPieceDarkColor();
  else if (piece == "Z")
    return getZPieceDarkColor();
  throw new Error("Invalid Value");
}
function trashPiece(ctx: CanvasRenderingContext2D, x: number, y: number) {
  const trash = trashColor();
  const trashBorder = trashBorderColor();
  const borderWidth = 2;
  ctx.strokeStyle = trashBorder;
  ctx.lineWidth = borderWidth;
  ctx.strokeRect(pieceWidth * x, pieceHeight * y,
    pieceWidth, pieceHeight)
  ctx.fillStyle = trash;
  ctx.fillRect(borderWidth + (pieceWidth * x), borderWidth + (pieceHeight * y),
    pieceWidth - (borderWidth * 2),
    pieceHeight - (borderWidth * 2));
}

function ghostPiece(ctx: CanvasRenderingContext2D, x: number, y: number) {
  const color = getGhostColor();
  const outerBorderWidth = 2;
  const innerBorderWidth = 2;
  ctx.strokeStyle = color;
  ctx.lineWidth = outerBorderWidth;
  ctx.strokeRect(pieceWidth * x, pieceHeight * y,
    pieceWidth, pieceHeight);
  ctx.lineWidth = innerBorderWidth;
  const offset = 9;
  ctx.strokeRect(offset + (pieceWidth * x), offset + (pieceHeight * y),
    pieceWidth - (offset * 2),
    pieceWidth - (offset * 2));
}


function clearCanvas(ctx: CanvasRenderingContext2D) {
  ctx.save();

  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, mainCanvas.width, mainCanvas.height);

  ctx.restore();
}


function drawBoard(board: string) {
  drawBufferBoard(board.substring(0, 200));
  drawMainBoard(board.substring(200, 400));
}


async function gameLost() {
  unlisteners.push(await listen(gameOverEmit, (e) => {
    lostEffect();

    if (currentGameOptions.number_of_players == 1) {
      setTimeout(() => {
        if (e.payload == true) {
          router.push("/again");
        } else {
          router.push("/stats");
        }
      }, 1500);
      removeInputListeners();
      unlisteners.forEach(el => {
        el()
      });
      unlisteners.length = 0;
    }
  }));
}

async function lineCleared() {
  unlisteners.push(await listen(lineClearedEmit, (e) => {
    lineClearedEffect(e.payload as ClearLinePattern);
  }));
}

async function pieceFixedEvent() {
  unlisteners.push(await listen(pieceFixed, () => {
    pieceFixedEffect();
  }));
}

async function gameWon() {
  unlisteners.push(await listen(gameWonEmit, () => {
    const $board = document.getElementById("wrap")! as HTMLElement;
    $board.classList.add("won");
    if (currentGameOptions.number_of_players == 1) {
      setTimeout(() => {
        router.push("/stats");
      }, 1500);
      unlisteners.forEach(el => {
        el()
      });
      unlisteners.length = 0;
      multiplayerUnlisteners.forEach(el => el());
      multiplayerUnlisteners.length = 0;
    }
    removeInputListeners();
  }));
}

async function lineClearedInfo() {
  unlisteners.push(await listen(lineClearedInfoEmit, (e) => {
    const $lines = document.getElementById("write-lines") as HTMLElement;
    $lines.innerText = e.payload as string;
  }));
}
async function pointsInfo() {
  unlisteners.push(await listen(pointsEmit, (e) => {
    const $points = document.getElementById("write-lines") as HTMLElement;
    $points.innerText = e.payload as string;
  }));
}
async function timer() {
  unlisteners.push(await listen(timeEmit, (e) => {
    const $points = document.getElementById("timer")! as HTMLElement;
    $points.innerText = e.payload as string;
  }));
}

async function boards() {
  multiplayerUnlisteners.push(await listen(stateEmitForOtherPlayers, (e) => {
    let player = e.payload as OtherPlayerState;
    let state = player.state.substring(200, 400);
    let board = document.getElementById("board-player-" + player.player.id)! as HTMLCanvasElement;
    drawMultiplayer(state, board.getContext("2d")!, false, board.width, board.height);
  }))
}

async function otherPlayerLost() {
  multiplayerUnlisteners.push(await listen(otherPlayerLostEmit, (e) => {
    let player = e.payload as Player;
    let board = document.getElementById("board-player-" + player.id) as HTMLCanvasElement;
    console.log("lost: board-player-" + player.id);
    if (board != null) {
      board.style.transform = "rotate(-15deg)";
      board.style.animation = "drop_board 1.5s ease-in forwards";
    }
  }))
}
async function otherPlayerWon() {
  console.log("event added");
  multiplayerUnlisteners.push(await listen(otherPlayerWonEmit, (e) => {
    let player = e.payload as WonSignal;
    console.log("won: board-player-" + player.player.id);

    let board = document.getElementById("board-player-" + player.player.id) as HTMLCanvasElement;
    console.log(board);
    if (board != null) {
      board.classList.add("won");
    } else {
      const $board = document.getElementById("wrap")! as HTMLElement;
      $board.classList.add("won");
    }
    if (player.is_hosting) {
      setTimeout(() => {
        router.push("/rehost");
      }, 1500);
    } else {
      setTimeout(() => {
        router.push("/rejoin");
      }, 1500);
    }

    unlisteners.forEach(el => el());
    unlisteners.length = 0;
    multiplayerUnlisteners.forEach(el => el());
    multiplayerUnlisteners.length = 0;
  }));
}

function drawMultiplayer(board: string, ctx: CanvasRenderingContext2D, drawLDivisories: boolean, width: number, height: number) {
  clearCanvas(ctx);
  if (drawLDivisories) {
    drawLines(ctx);
  }
  let pieceWidth = width / 10;
  let pieceHeight = height / 20;
  for (let i = boardSize - 1; i > -1; i--) {
    const piece: Piece = board[i]! as Piece;
    if (piece == Piece.Empty)
      continue;
    const y = Math.floor(i / columnNumber);
    const x = i % columnNumber;

    if (piece == Piece.Ghost) {
      continue;
    }

    let color;
    if (piece == Piece.Trash) {
      color = trashColor();
    } else {
      color = getColor(piece) != "transparent" ? getColor(piece) : getDarkColor(piece)
    }

    ctx.fillStyle = color;
    ctx.fillRect(pieceWidth * x, pieceHeight * y, pieceWidth, pieceHeight);

  }
}

