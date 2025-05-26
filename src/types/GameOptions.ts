/**
 * `GameOptions` defines the options for a game session, used for communication with the backend.
 */
export type GameOptions =  {
    /**
     * The number of players in the game.
     */
    number_of_players: number,
    /**
     * Indicates whether the game is in 40-lines mode.
     */
    lines_40: boolean
    /**
     * Indicates whether the game is in blitz mode.
     */
    blitz: boolean
    /**
     * Indicates whether the game is in normal mode.
     */
    normal: boolean
};
