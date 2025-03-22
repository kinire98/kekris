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
import type { GameOptions } from "../types";
import { gameWonEffect, lineClearedEffect, lostEffect, pieceFixedEffect } from "./effects";
import type { ClearLinePattern } from "../types/ClearLinePattern";


const canvasHeight = 760;
const canvasWidth = 380;

const columnNumber = 10;
const rowNumber = 20;

const spacing = 38;
const pieceWidth = 38;
const pieceHeight = 38;


const boardSize = columnNumber * rowNumber;
const boardStateEmit = "board_state_emit";
const lineClearedEmit = "line_cleared";
const pieceFixed = "piece_fixed";
const pointsEmit = "points";
const gameOverEmit = "game_over";
const gameWonEmit = "game_won";
const lineClearedInfoEmit = "line_cleared_info";
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

export default function startDraw(canvas: HTMLCanvasElement, secondCanvas: HTMLCanvasElement) {
  const ctx: CanvasRenderingContext2D = canvas.getContext("2d")!;
  mainCanvas = canvas;
  bufferCanvas = secondCanvas;
  drawLines(ctx);
  startBoardChangeEventListener();
  let options: GameOptions = {
    number_of_players: 1,
    lines_40: false,
    normal: true,
    blitz: false
  };
  invoke("start_game", {
    options: options
  });
  gameLost();
  lineCleared();
  pieceFixedEvent();
  gameWon();
  lineClearedInfo();
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
    const piece: string = board[i]!;
    if (piece == "E")
      continue;
    const y = Math.floor(i / columnNumber);
    const x = i % columnNumber;
    if (piece == "R") {
      trashPiece(ctx, x, y);
      continue;
    }
    if (piece == "G") {
      ghostPiece(ctx, x, y);
      continue;
    }
    const color = getColor(piece);
    const darkColor = getDarkColor(piece);
    ctx.strokeStyle = color;
    ctx.lineWidth = 8; // central rect
    //First magic number -> x offset
    //Second magic number -> y offset
    //Third magic number -> width
    //Fourth magic number -> height
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
  console.log(piece);
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
  await listen(gameOverEmit, () => {
    lostEffect();
  });
}

async function lineCleared() {
  await listen(lineClearedEmit, (e) => {
    lineClearedEffect(e.payload as ClearLinePattern);
  });
}

async function pieceFixedEvent() {
  await listen(pieceFixed, () => {
    pieceFixedEffect();
  });
}

async function gameWon() {
  await listen(gameWonEmit, () => {
    gameWonEffect();
  });
}

async function lineClearedInfo() {
  await listen(lineClearedInfoEmit, (e) => {
    const $lines = document.getElementById("write-lines") as HTMLElement;
    $lines.innerHTML = e.payload as string;
  })
}
