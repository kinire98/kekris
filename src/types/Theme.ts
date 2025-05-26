/**
 * `Theme` defines the color scheme for the game, specifying the appearance of each piece.
 */
export type Theme = {
    /**
     * Theme for the I piece.
     */
    I: PieceTheme,
    /**
     * Theme for the O piece.
     */
    O: PieceTheme,
    /**
     * Theme for the T piece.
     */
    T: PieceTheme,
    /**
     * Theme for the S piece.
     */
    S: PieceTheme,
    /**
     * Theme for the Z piece.
     */
    Z: PieceTheme,
    /**
     * Theme for the L piece.
     */
    L: PieceTheme,
    /**
     * Theme for the J piece.
     */
    J: PieceTheme,
}
/**
 * `PieceTheme` defines the colors used to render a single piece.
 */
export type PieceTheme = {
    /**
     * Border color of the piece.
     */
    border: string,
    /**
     * Fill color of the piece.
     */
    fill: string,
}
/**
 * `ColorType` is a type alias that represents the possible color properties of a piece.
 */
export type ColorType = "border" | "fill";