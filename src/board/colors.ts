// E -> Empty
// C -> Clear
// O -> yellow
// I -> cyan
// T -> Purple
// L -> orange
// J -> blue
// S -> green
// Z -> red

import { getTheme } from "../helpers/themes";
import { Piece } from "../types/Piece";
import { ColorType } from "../types/Theme";
import { iceTheme } from "./themes/iceTheme";
import { monochromeTheme } from "./themes/monochromeTheme";
import { neonTheme } from "./themes/neonTheme";
import { pastelTheme } from "./themes/pastelTheme";
import { retroNeonTheme } from "./themes/retroNeonTheme";
import { mutedMineralsTheme } from "./themes/mutedMineralsTheme";

/**
 * Defines the available themes for the game.
 * Each theme is an object that maps a theme name to a `Theme` object.
 */
export const themes = {
    "neon": neonTheme,
    "pastel": pastelTheme,
    "monochrome": monochromeTheme,
    "retroNeon": retroNeonTheme,
    "ice": iceTheme,
    "mutedMinerals": mutedMineralsTheme
}

/**
 * Represents the possible theme names that can be used in the game.
 */
export type PossibleThemes = keyof typeof themes;

let storedTheme: PossibleThemes = getTheme()! as PossibleThemes;
const defaultTheme: PossibleThemes = "neon";

/**
 * Gets the border color for the game board.
 * @returns The border color as a string.
 */
export function getBorderColor(): string {
    return "#404040"
}

/**
 * Gets the border color for trash pieces.
 * @returns The trash border color as a string.
 */
export function trashBorderColor(): string {
    return "#505050";
}

/**
 * Gets the fill color for trash pieces.
 * @returns The trash color as a string.
 */
export function trashColor(): string {
    return "#303030";
}

/**
 * Gets the color for ghost pieces.
 * @returns The ghost color as a string.
 */
export function getGhostColor(): string {
    return "#f0f0f0";
}

/**
 * Gets the fill color for O pieces.
 * @returns The fill color for O pieces as a string.
 */
export function getOPieceColor(): string {
    return getColor(Piece.O, "fill");
}
/**
 * Gets the border color for O pieces.
 * @returns The border color for O pieces as a string.
 */
export function getOPieceDarkColor(): string {
    return getColor(Piece.O, "border");
}
/**
 * Gets the fill color for I pieces.
 */
export function getIPieceColor(): string {
    return getColor(Piece.I, "fill");
}
/**
 * Gets the border color for I pieces.
 */
export function getIPieceDarkColor(): string {
    return getColor(Piece.I, "border");
}
/**
 * Gets the fill color for T pieces.
 */
export function getTPieceColor(): string {
    return getColor(Piece.T, "fill");
}
/**
 * Gets the border color for T pieces.
 */
export function getTPieceDarkColor(): string {
    return getColor(Piece.T, "border");
}
/**
 * Gets the fill color for L pieces.
 */
export function getLPieceColor(): string {
    return getColor(Piece.L, "fill");
}
/**
 * Gets the border color for L pieces.
 */
export function getLPieceDarkColor(): string {
    return getColor(Piece.L, "border");
}
/**
 * Gets the fill color for J pieces.
 */
export function getJPieceColor(): string {
    return getColor(Piece.J, "fill");
}
/**
 * Gets the border color for J pieces.
 */
export function getJPieceDarkColor(): string {
    return getColor(Piece.J, "border");
}
/**
 * Gets the fill color for S pieces.
 */
export function getSPieceColor(): string {
    return getColor(Piece.S, "fill");
}
/**
 * Gets the border color for S pieces.
 */
export function getSPieceDarkColor(): string {
    return getColor(Piece.S, "border");
}
/**
 * Gets the fill color for Z pieces.
 */
export function getZPieceColor(): string {
    return getColor(Piece.Z, "fill");
}
/**
 * Gets the border color for Z pieces.
 */
export function getZPieceDarkColor(): string {
    return getColor(Piece.Z, "border");
}

/**
 * Gets the color for a piece based on the current theme.
 * @param piece The piece type.
 * @param colorType The color type (fill or border).
 * @returns The color for the piece as a string.
 */
function getColor(piece: Piece, colorType: ColorType): string {
    if (piece == Piece.Ghost) {
        return "#f0f0f0";
    }
    if (piece == Piece.Trash) {
        if (colorType == "border") {
            return "#505050";

        } else {
            return "#303030";
        }
    }
    if (piece == Piece.Empty) {
        return "transparent";
    }
    let theme = getTheme();
    if (theme == null) {
        storedTheme = defaultTheme;
        return themes[defaultTheme][piece][colorType];
    }
    storedTheme = theme as PossibleThemes;
    return themes[storedTheme][piece][colorType];
}