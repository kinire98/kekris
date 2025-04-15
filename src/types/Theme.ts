export type Theme = {
    I: PieceTheme,
    O: PieceTheme,
    T: PieceTheme,
    S: PieceTheme,
    Z: PieceTheme,
    L: PieceTheme,
    J: PieceTheme,
}
export type PieceTheme = {
    border: string,
    fill: string,
}
export type ColorType = "border" | "fill";