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




export const themes = {
    "neon": neonTheme,
    "pastel": pastelTheme,
    "monochrome": monochromeTheme,
    "retroNeon": retroNeonTheme,
    "ice": iceTheme,
    "mutedMinerals": mutedMineralsTheme
}

export type PossibleThemes = keyof typeof themes;

let storedTheme: PossibleThemes = getTheme()! as PossibleThemes;
const defaultTheme: PossibleThemes = "neon";

export function getBorderColor(): string {
    return "#404040"
}

export function trashBorderColor(): string {
    return "#505050";
}

export function trashColor(): string {
    return "#303030";
}

export function getGhostColor(): string {
    return "#f0f0f0";
}

export function getOPieceColor(): string {
    return getColor(Piece.O, "fill");
}
export function getOPieceDarkColor(): string {
    return getColor(Piece.O, "border");
}
export function getIPieceColor(): string {
    return getColor(Piece.I, "fill");
}
export function getIPieceDarkColor(): string {
    return getColor(Piece.I, "border");
}
export function getTPieceColor(): string {
    return getColor(Piece.T, "fill");
}
export function getTPieceDarkColor(): string {
    return getColor(Piece.T, "border");
}
export function getLPieceColor(): string {
    return getColor(Piece.L, "fill");
}
export function getLPieceDarkColor(): string {
    return getColor(Piece.L, "border");
}
export function getJPieceColor(): string {
    return getColor(Piece.J, "fill");
}
export function getJPieceDarkColor(): string {
    return getColor(Piece.J, "border");
}
export function getSPieceColor(): string {
    return getColor(Piece.S, "fill");
}
export function getSPieceDarkColor(): string {
    return getColor(Piece.S, "border");
}
export function getZPieceColor(): string {
    return getColor(Piece.Z, "fill");
}
export function getZPieceDarkColor(): string {
    return getColor(Piece.Z, "border");
}

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