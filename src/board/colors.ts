// E -> Empty
// C -> Clear
// O -> yellow
// I -> cyan
// T -> Purple
// L -> orange
// J -> blue
// S -> green
// Z -> red

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
    return getColor();
}
export function getOPieceDarkColor(): string {
    return getDarkColor();
}
export function getIPieceColor(): string {
    return getColor();
}
export function getIPieceDarkColor(): string {
    return getDarkColor();
}
export function getTPieceColor(): string {
    return getColor();
}
export function getTPieceDarkColor(): string {
    return getDarkColor();
}
export function getLPieceColor(): string {
    return getColor();
}
export function getLPieceDarkColor(): string {
    return getDarkColor();
}
export function getJPieceColor(): string {
    return getColor();
}
export function getJPieceDarkColor(): string {
    return getDarkColor();
}
export function getSPieceColor(): string {
    return getColor();
}
export function getSPieceDarkColor(): string {
    return getDarkColor();
}
export function getZPieceColor(): string {
    return getColor();
}
export function getZPieceDarkColor(): string {
    return getDarkColor();
}
function getColor(): string {
    return "transparent";
}
function getDarkColor(): string {
    return "#00ADB5";
}