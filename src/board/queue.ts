import { listen } from "@tauri-apps/api/event";
import { getIPieceColor, getIPieceDarkColor, getJPieceColor, getJPieceDarkColor, getLPieceColor, getLPieceDarkColor, getOPieceColor, getOPieceDarkColor, getSPieceColor, getSPieceDarkColor, getTPieceColor, getTPieceDarkColor, getZPieceColor, getZPieceDarkColor } from "./colors";
import { Piece } from "../types/piece";

const queueEmit = "queue_emit";

let canvas: HTMLCanvasElement;

const pieceWidth = 34;
const pieceHeight = 34;
export default function startQueue(queueCanvas: HTMLCanvasElement) {
    canvas = queueCanvas;
    listenToQueue();
}
async function listenToQueue() {
    await listen<Piece[]>(queueEmit, (e) => {
        drawQueue(e.payload);
    });
}

function drawQueue(pieces: Piece[]) {
    const ctx: CanvasRenderingContext2D = canvas.getContext("2d")!;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < pieces.length; i++) {
        draw(pieces[i]!, i, ctx);
    }
}

function draw(piece: Piece, position: number, ctx: CanvasRenderingContext2D) {
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
            start_y = canvas.height / 10 - pieceHeight
            break;
        case Piece.I:
            start_y = canvas.height / 10 - pieceHeight * 0.5
            break;
        default:
            throw new Error(`${piece} shouldn't be sent through here`);
    }

    switch (piece) {
        case Piece.T:
            drawPiece(start_x + pieceWidth, start_y, getTPieceColor(), getTPieceDarkColor(), ctx, position);
            drawPiece(start_x, start_y + pieceHeight, getTPieceColor(), getTPieceDarkColor(), ctx, position);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getTPieceColor(), getTPieceDarkColor(), ctx, position);
            drawPiece(start_x + (pieceWidth * 2), start_y + pieceHeight, getTPieceColor(), getTPieceDarkColor(), ctx, position);
            break;
        case Piece.I:
            drawPiece(start_x, start_y, getIPieceColor(), getIPieceDarkColor(), ctx, position);
            drawPiece(start_x + pieceWidth, start_y, getIPieceColor(), getIPieceDarkColor(), ctx, position);
            drawPiece(start_x + (pieceWidth * 2), start_y, getIPieceColor(), getIPieceDarkColor(), ctx, position);
            drawPiece(start_x + (pieceWidth * 3), start_y, getIPieceColor(), getIPieceDarkColor(), ctx, position);
            break;
        case Piece.J:
            drawPiece(start_x, start_y, getJPieceColor(), getJPieceDarkColor(), ctx, position);
            drawPiece(start_x, start_y + pieceHeight, getJPieceColor(), getJPieceDarkColor(), ctx, position);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getJPieceColor(), getJPieceDarkColor(), ctx, position);
            drawPiece(start_x + (pieceWidth * 2), start_y + pieceHeight, getJPieceColor(), getJPieceDarkColor(), ctx, position);
            break;
        case Piece.L:
            drawPiece(start_x + (pieceWidth * 2), start_y, getLPieceColor(), getLPieceDarkColor(), ctx, position);
            drawPiece(start_x, start_y + pieceHeight, getLPieceColor(), getLPieceDarkColor(), ctx, position);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getLPieceColor(), getLPieceDarkColor(), ctx, position);
            drawPiece(start_x + (pieceWidth * 2), start_y + pieceHeight, getLPieceColor(), getLPieceDarkColor(), ctx, position);
            break;
        case Piece.O:
            drawPiece(start_x, start_y, getOPieceColor(), getOPieceDarkColor(), ctx, position);
            drawPiece(start_x, start_y + pieceHeight, getOPieceColor(), getOPieceDarkColor(), ctx, position);
            drawPiece(start_x + pieceWidth, start_y, getOPieceColor(), getOPieceDarkColor(), ctx, position);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getOPieceColor(), getOPieceDarkColor(), ctx, position);
            break;
        case Piece.S:
            drawPiece(start_x + pieceWidth, start_y, getSPieceColor(), getSPieceDarkColor(), ctx, position);
            drawPiece(start_x + (pieceWidth * 2), start_y, getSPieceColor(), getSPieceDarkColor(), ctx, position);
            drawPiece(start_x, start_y + pieceHeight, getSPieceColor(), getSPieceDarkColor(), ctx, position);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getSPieceColor(), getSPieceDarkColor(), ctx, position);
            break;
        case Piece.Z:
            drawPiece(start_x + pieceWidth, start_y, getZPieceColor(), getZPieceDarkColor(), ctx, position);
            drawPiece(start_x + (pieceWidth * 2), start_y, getZPieceColor(), getZPieceDarkColor(), ctx, position);
            drawPiece(start_x, start_y + pieceHeight, getZPieceColor(), getZPieceDarkColor(), ctx, position);
            drawPiece(start_x + pieceWidth, start_y + pieceHeight, getZPieceColor(), getZPieceDarkColor(), ctx, position);
            break;
        default:
            throw new Error(`${piece} shouldn't be sent through here`);
    }
}
function drawPiece(x: number, y: number, color: string, darkColor: string, ctx: CanvasRenderingContext2D, position: number) {
    const widthSecondRing = 5;
    y = ((canvas.height / 5) * position) + y;
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