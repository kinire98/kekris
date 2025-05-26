import { Piece } from "../types/Piece";
import { PieceTheme, Theme } from "../types/Theme";

const pieceWidth = 30;
const pieceHeight = 30;
const width = 700;
const margin = (width - ((pieceWidth * 3) * 5 + (pieceWidth * 2) + (pieceWidth * 4))) / 7;
const height = 150;
const start_y = height / 2 - pieceHeight;

/**
 * Draws a theme test pattern on the canvas.
 * @param canvas The canvas element to draw on.
 * @param theme The theme to use.
 */
export default function drawThemeTest(canvas: HTMLCanvasElement, theme: Theme) {
    const ctx = canvas.getContext("2d")!;
    let start_x = margin * 2.95;
    drawPiece(ctx, start_x, start_y, Piece.O, theme.O);
    start_x += (pieceWidth * 2) + margin;
    drawPiece(ctx, start_x, start_y, Piece.I, theme.I);
    start_x += (pieceWidth * 4) + margin;
    drawPiece(ctx, start_x, start_y, Piece.J, theme.J);
    start_x += (pieceWidth * 3) + margin;
    drawPiece(ctx, start_x, start_y, Piece.L, theme.L);
    start_x += (pieceWidth * 3) + margin;
    drawPiece(ctx, start_x, start_y, Piece.Z, theme.Z);
    start_x += (pieceWidth * 3) + margin;
    drawPiece(ctx, start_x, start_y, Piece.S, theme.S);
    start_x += (pieceWidth * 3) + margin;
    drawPiece(ctx, start_x, start_y, Piece.T, theme.T);
}

/**
 * Draws a single piece on the canvas.
 * @param ctx The canvas rendering context.
 * @param start_piece_x The x coordinate of the top-left corner of the piece.
 * @param start_piece_y The y coordinate of the top-left corner of the piece.
 * @param piece The piece to draw.
 * @param theme The theme to use.
 */
function drawPiece(
    ctx: CanvasRenderingContext2D,
    start_piece_x: number,
    start_piece_y: number,
    piece: Piece,
    theme: PieceTheme
) {
    switch (piece) {
        case Piece.I:
            start_piece_y += pieceHeight / 2;
            drawSquare(ctx, start_piece_x, start_piece_y, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y, theme);
            drawSquare(ctx, start_piece_x + pieceWidth * 2, start_piece_y, theme);
            drawSquare(ctx, start_piece_x + pieceWidth * 3, start_piece_y, theme);
            break;
        case Piece.O:
            drawSquare(ctx, start_piece_x, start_piece_y, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y, theme);
            drawSquare(ctx, start_piece_x, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y + pieceHeight, theme);
            break;
        case Piece.L:
            drawSquare(ctx, start_piece_x + pieceWidth * 2, start_piece_y, theme);
            drawSquare(ctx, start_piece_x, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth * 2, start_piece_y + pieceHeight, theme);
            break;
        case Piece.J:
            drawSquare(ctx, start_piece_x, start_piece_y, theme);
            drawSquare(ctx, start_piece_x, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth * 2, start_piece_y + pieceHeight, theme);
            break;
        case Piece.Z:
            drawSquare(ctx, start_piece_x, start_piece_y, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth * 2, start_piece_y + pieceHeight, theme);
            break;
        case Piece.S:
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y, theme);
            drawSquare(ctx, start_piece_x + pieceWidth * 2, start_piece_y, theme);
            drawSquare(ctx, start_piece_x, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y + pieceHeight, theme);
            break;
        case Piece.T:
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y, theme);
            drawSquare(ctx, start_piece_x, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth, start_piece_y + pieceHeight, theme);
            drawSquare(ctx, start_piece_x + pieceWidth * 2, start_piece_y + pieceHeight, theme);
            break;
        default:
            throw new Error("Shouldn't be here");
    }
}

/**
 * Draws a single square on the canvas.
 * @param ctx The canvas rendering context.
 * @param x The x coordinate of the top-left corner of the square.
 * @param y The y coordinate of the top-left corner of the square.
 * @param theme The theme to use.
 */
function drawSquare(
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    theme: PieceTheme
) {
    ctx.strokeStyle = theme.fill;
    ctx.lineWidth = 7;
    ctx.strokeRect(x + 5, y + 5, pieceWidth - 9, pieceHeight - 9);
    ctx.fillStyle = theme.fill;
    ctx.fillRect(x + 9, y + 9, pieceWidth - 17, pieceHeight - 17);

    ctx.strokeStyle = theme.border;
    ctx.lineWidth = 2;
    ctx.strokeRect(x + 1, y + 1, pieceWidth - 1, pieceHeight - 1);
    ctx.strokeRect(x + 8, y + 8, pieceWidth - 15, pieceHeight - 15);

}