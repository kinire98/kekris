/**
 * `ClearLinePattern` represents the different patterns of lines that can be cleared in the game.
 * This enum is used for communication with the backend to represent the type of line clear that occurred.
 */
export enum ClearLinePattern {
    None = "None",
    Single = "Single",
    Double = "Double",
    Triple = "Triple",
    Tetris = "Tetris",
    TSpin = "TSpin",
    TSpinSingle = "TSpinSingle",
    TSpinDouble = "TSpinDouble",
    TSpinTriple = "TSpinTriple",
    MiniTSpin = "MiniTSpin",
    MiniTSpinSingle = "MiniTSpinSingle",
}
