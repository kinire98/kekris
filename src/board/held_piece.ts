import { listen } from "@tauri-apps/api/event";
import {
    getIPieceColor, getIPieceDarkColor, getJPieceColor,
    getJPieceDarkColor, getLPieceColor, getLPieceDarkColor,
    getOPieceColor, getOPieceDarkColor, getSPieceColor,
    getSPieceDarkColor, getTPieceColor, getTPieceDarkColor,
    getZPieceColor, getZPieceDarkColor
} from "./colors";
import { Piece } from "../types/Piece";

const heldPieceEmit = "held_piece_emit";

const pieceWidth = 34;
const pieceHeight = 34;

let canvas: HTMLCanvasElement;

/**
 * Initializes the held piece canvas and sets up the event listener.
 * @param heldPieceCanvas The canvas element for the held piece.
 */
export default function startHeld(heldPieceCanvas: HTMLCanvasElement) {
    canvas = heldPieceCanvas;
    listener();
}

/**
 * Sets up the event listener for the held piece emit event.
 */
async function listener() {
    await listen<Piece>(heldPieceEmit, (e) => {
        draw(e.payload);
    });
}

/**
 * Draws the held piece on the canvas.
 * @param piece The piece to draw.
 */
function draw(piece: Piece) {
    let start_x;
    let start_y;
    switch (piece) {
        case Piece.T:
        case Piece.L:
        case Piece.J:
        case Piece.S:
        case Piece.Z:
            start_x = canvas.width / 2 - pieceWidth * 1.5
            break;
        case Piece.I:
            start_x = canvas.width / 2 - pieceWidth * 2
            break;
        case Piece.O:
            start_x = canvas.width / 2 - pieceWidth
            break;
        default:
            throw new Error(`${piece} shouldn't be sent through here`);
    }
    switch (piece) {
        case Piece.T:
        case Piece.L:
        case Piece.J:
        case Piece.S:
        case Piece.Z:
        case Piece.O:
            start_y = canvas.height / 2 - pieceHeight
            break;
        case Piece.I:
            start_y = canvas.height / 2 - pieceHeight * 0.5
            break;
        default:
            throw new Error(`${piece} shouldn't be sent through here`);
    }

    const ctx: CanvasRenderingContext2D = canvas.getContext("2d")!;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    switch (piece) {
        case Piece.T:
            drawPiece(start_x + pieceWidth, start_y, getTPieceColor(), getTPieceDarkColor(), ctx);
            drawPiece(start_x, start_y + pieceHeight, getTPieceColor(), getTPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getTPieceColor(), getTPieceDarkColor(), ctx);
            drawPiece(start_x + (pieceWidth * 2), start_y + pieceHeight, getTPieceColor(), getTPieceDarkColor(), ctx);
            break;
        case Piece.I:
            drawPiece(start_x, start_y, getIPieceColor(), getIPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y, getIPieceColor(), getIPieceDarkColor(), ctx);
            drawPiece(start_x + (pieceWidth * 2), start_y, getIPieceColor(), getIPieceDarkColor(), ctx);
            drawPiece(start_x + (pieceWidth * 3), start_y, getIPieceColor(), getIPieceDarkColor(), ctx);
            break;
        case Piece.J:
            drawPiece(start_x, start_y, getJPieceColor(), getJPieceDarkColor(), ctx);
            drawPiece(start_x, start_y + pieceHeight, getJPieceColor(), getJPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getJPieceColor(), getJPieceDarkColor(), ctx);
            drawPiece(start_x + (pieceWidth * 2), start_y + pieceHeight, getJPieceColor(), getJPieceDarkColor(), ctx);
            break;
        case Piece.L:
            drawPiece(start_x + (pieceWidth * 2), start_y, getLPieceColor(), getLPieceDarkColor(), ctx);
            drawPiece(start_x, start_y + pieceHeight, getLPieceColor(), getLPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getLPieceColor(), getLPieceDarkColor(), ctx);
            drawPiece(start_x + (pieceWidth * 2), start_y + pieceHeight, getLPieceColor(), getLPieceDarkColor(), ctx);
            break;
        case Piece.O:
            drawPiece(start_x, start_y, getOPieceColor(), getOPieceDarkColor(), ctx);
            drawPiece(start_x, start_y + pieceHeight, getOPieceColor(), getOPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y, getOPieceColor(), getOPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getOPieceColor(), getOPieceDarkColor(), ctx);
            break;
        case Piece.S:
            drawPiece(start_x + pieceWidth, start_y, getSPieceColor(), getSPieceDarkColor(), ctx);
            drawPiece(start_x + (pieceWidth * 2), start_y, getSPieceColor(), getSPieceDarkColor(), ctx);
            drawPiece(start_x, start_y + pieceHeight, getSPieceColor(), getSPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getSPieceColor(), getSPieceDarkColor(), ctx);
            break;
        case Piece.Z:
            drawPiece(start_x, start_y, getZPieceColor(), getZPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y, getZPieceColor(), getZPieceDarkColor(), ctx);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getZPieceColor(), getZPieceDarkColor(), ctx);
            drawPiece(start_x + (pieceWidth * 2), start_y + pieceHeight, getZPieceColor(), getZPieceDarkColor(), ctx);
            break;
        default:
            throw new Error(`${piece} shouldn't be sent through here`);
    }
}

/**
 * Draws a single piece block on the canvas.
 * @param x The x coordinate of the piece.
 * @param y The y coordinate of the piece.
 * @param color The color of the piece.
 * @param darkColor The dark color of the piece.
 * @param ctx The canvas rendering context.
 */
function drawPiece(x: number, y: number, color: string, darkColor: string, ctx: CanvasRenderingContext2D) {
    const widthSecondRing = 5;

    ctx.lineWidth = 8;
    ctx.strokeStyle = color;
    ctx.strokeRect(widthSecondRing + x, widthSecondRing + y, pieceWidth - (widthSecondRing * 2), pieceHeight - (widthSecondRing * 2));
    ctx.strokeStyle = darkColor;
    ctx.lineWidth = 2;
    ctx.strokeRect(1 + x, 1 + y, pieceWidth - 1, pieceHeight - 1);
    ctx.strokeStyle = darkColor;
    ctx.lineWidth = 2;
    ctx.strokeRect(9 + x, 9 + y, pieceWidth / 2, pieceHeight / 2);

    ctx.fillStyle = color;
    ctx.fillRect(10 + x, 10 + y, pieceWidth / 3 + 3, pieceHeight / 3 + 3);
}
