export interface EmitGameInfo {
    last_game_info: GameInfo;
    top_five_results: GameInfo[];
    last_in_top_five: number;
}

export interface GameInfo {
    piece_moves: number;
    spins: number;
    lines_cleared: number;
    pieces_used: number;
    singles: number;
    doubles: number;
    triples: number;
    tetrises: number;
    tspins: number;
    tspin_singles: number;
    tspin_doubles: number;
    tspin_triples: number;
    minitspins: number;
    minitspin_singles: number;
    specific_info: GameTypeInfo;
}

export type GameTypeInfo =
    | { Classic: ClassicGameInfo }
    | { Lines: LinesGameInfo }
    | { Blitz: BlitzGameInfo };

export interface ClassicGameInfo {
    time_endured: number;
    points: number;
    level_reached: number;
}

export interface LinesGameInfo {
    time_endured: number;
}

export interface BlitzGameInfo {
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
