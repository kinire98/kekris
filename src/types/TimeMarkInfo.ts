/**
 * `TimeMarkInfo` represents a time mark in a replay, indicating a specific point in the game.
 */
export type TimeMarkInfo = {
    /**
     * The position of the time mark in the replay (e.g., frame number).
     */
    position: number,
    /**
     * A descriptive mark or label for the time point.
     */
    mark: string,
    /**
     * Indicates whether this time mark corresponds to the last played position.
     */
    its_last_played: boolean
};