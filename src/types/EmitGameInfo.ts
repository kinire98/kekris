/**
 * `EmitGameInfo` encapsulates game information for transmission to the UI.
 * It includes the last game's details, top scores, and status.
 */
export interface EmitGameInfo {
    /**
     * Details of the most recently played game.
     */
    last_game_info: GameInfo;
    /**
     * An array containing the top five game results.
     */
    top_five_results: GameInfo[];
    /**
     * Index of the last game within the top five results (-1 if not present).
     */
    last_in_top_five: number;
    /**
     * Indicates whether the data is empty or not.
     */
    empty: boolean;
}

/**
 * `GameInfo` provides general statistics for a single game session.
 */
export interface GameInfo {
    /**
     * Number of times a piece was moved.
     */
    piece_moves: number;
    /**
     * Number of spins performed.
     */
    spins: number;
    /**
     * Number of lines cleared.
     */
    lines_cleared: number;
    /**
     * Number of pieces used in the game.
     */
    pieces_used: number;
    /**
     * Number of single line clears.
     */
    singles: number;
    /**
     * Number of double line clears.
     */
    doubles: number;
    /**
     * Number of triple line clears.
     */
    triples: number;
    /**
     * Number of Tetrises (four-line clears).
     */
    tetrises: number;
    /**
     * Number of T-spins performed.
     */
    tspins: number;
    /**
     * Number of T-spin single line clears.
     */
    tspin_singles: number;
    /**
     * Number of T-spin double line clears.
     */
    tspin_doubles: number;
    /**
     * Number of T-spin triple line clears.
     */
    tspin_triples: number;
    /**
     * Number of mini T-spins performed.
     */
    minitspins: number;
    /**
     * Number of mini T-spin single line clears.
     */
    minitspin_singles: number;
    /**
     * Additional specific information about the game type.
     */
    specific_info: GameTypeInfo;
}

/**
 * `GameTypeInfo` is a discriminated union that represents game-specific information.
 */
export type GameTypeInfo =
    | { Classic: ClassicGameInfo }
    | { Lines: LinesGameInfo }
    | { Blitz: BlitzGameInfo };

/**
 * `ClassicGameInfo` provides details specific to a classic Tetris game.
 */
export interface ClassicGameInfo {
    /**
     * Time endured in the game (in seconds).
     */
    time_endured: number;
    /**
     * Points scored in the game.
     */
    points: number;
    /**
     * Level reached during the game.
     */
    level_reached: number;
}

/**
 * `LinesGameInfo` provides details specific to a 40-lines Tetris game.
 */
export interface LinesGameInfo {
    /**
     * Time endured in the game (in seconds).
     */
    time_endured: number;
}

/**
 * `BlitzGameInfo` provides details specific to a Blitz Tetris game.
 */
export interface BlitzGameInfo {
    /**
     * Points scored in the game.
     */
    points: number;
}
export function isClassic(info: GameTypeInfo): info is { Classic: ClassicGameInfo } {
    return "Classic" in info;
}

export function isLines(info: GameTypeInfo): info is { Lines: LinesGameInfo } {
    return "Lines" in info;
}

export function isBlitz(info: GameTypeInfo): info is { Blitz: BlitzGameInfo } {
    return "Blitz" in info;
}
