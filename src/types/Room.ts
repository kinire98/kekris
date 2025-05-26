/**
 * `RoomInfo` represents the information about a game room, used for displaying available rooms.
 */
export type RoomInfo = {
    /**
     * The current number of players in the room.
     */
    number_of_players: number;
    /**
     * The maximum limit of players allowed in the room.
     */
    limit_of_players: number;
    /**
     * The name of the room.
     */
    name: string;
    /**
     * The number of games played in this room.
     */
    games_played: number;
    /**
     * The IP address of the room.
     */
    ip: string;
};

/**
 * `Visibility` represents the visibility setting of a game room.
 */
export type Visibility = "LocalNetwork" | "Internet";

/**
 * `Player` represents a player in a game room.
 */
export type Player = {
    /**
     * The unique identifier of the player.
     */
    id: number;
    /**
     * The name of the player.
     */
    name: string;
    /**
     * The IP address of the player.
     */
    ip: string;
    /**
     * The number of games won by the player.
     */
    games_won: number;
    /**
     * Indicates whether the player is currently playing.
     */
    playing: boolean;
    /**
     * The last time the player was active.
     */
    last_time: number;
    /**
     * The ping of the player, indicating network latency.
     */
    ping: number;
};

/**
 * `Room` represents a game room with its associated properties.
 */
export type Room = {
    /**
     * An array of players currently in the room.
     */
    players: Player[];
    /**
     * The visibility setting of the room.
     */
    visibility: Visibility;
    /**
     * The name of the room.
     */
    name: string;
    /**
     * The maximum limit of players allowed in the room.
     */
    limit_of_players: number;
    /**
     * The number of games played in this room.
     */
    games_played: number;
};
/**
 * `OtherPlayerState` represents the state of another player's game board.
 */
export type OtherPlayerState = {
    /**
     * The player's information.
     */
    player: Player;
    /**
     * The current state of the player's game board.
     */
    state: string;
};
/**
 * `WonSignal` represents a signal indicating that a player has won the game.
 */
export type WonSignal = {
    /**
     * The player who won the game.
     */
    player: Player;
    /**
     * Indicates whether the player is hosting the game.
     */
    is_hosting: boolean;
};

