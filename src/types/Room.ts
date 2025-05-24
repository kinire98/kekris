export type RoomInfo = {
    number_of_players: number;
    limit_of_players: number;
    name: string;
    games_played: number;
    ip: string;
};

export type Visibility = "LocalNetwork" | "Internet";

export type Player = {
    id: number;
    name: string;
    ip: string;
    games_won: number;
    playing: boolean;
    last_time: number;
    ping: number;
};

export type Room = {
    players: Player[];
    visibility: Visibility;
    name: string;
    limit_of_players: number;
    games_played: number;
};
export type OtherPlayerState = {
    player: Player;
    state: string;
};
export type WonSignal = {
    player: Player;
    is_hosting: boolean;
};

