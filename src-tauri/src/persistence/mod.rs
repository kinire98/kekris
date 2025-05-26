use tokio::sync::OnceCell;

pub mod migrations;

pub mod store_game_info;

pub mod retreive_game_info;

static DB_URL: OnceCell<String> = OnceCell::const_new();

/// Name of the games table in the database.
const GAME_TABLE_NAME: &str = "games";
/// Name of the classic game info table in the database.
const CLASSIC_TABLE_NAME: &str = "classic";
/// Name of the blitz game info table in the database.
const BLITZ_TABLE_NAME: &str = "blitz";
/// Name of the lines game info table in the database.
const LINES_TABLE_NAME: &str = "lines";
/// Name of the game info table in the database.
const GAME_INFO_TABLE_NAME: &str = "game_info";

/// Column name for piece moves in the game info table.
const GAME_INFO_PIECE_MOVES: &str = "piece_moves";
/// Column name for spins in the game info table.
const GAME_INFO_SPINS: &str = "spins";
/// Column name for lines cleared in the game info table.
const GAME_INFO_LINES_CLEARED: &str = "lines_cleared";
/// Column name for pieces used in the game info table.
const GAME_INFO_PIECES_USED: &str = "pieces_used";
/// Column name for singles in the game info table.
const GAME_INFO_SINGLES: &str = "singles";
/// Column name for doubles in the game info table.
const GAME_INFO_DOUBLES: &str = "doubles";
/// Column name for triples in the game info table.
const GAME_INFO_TRIPLES: &str = "triples";
/// Column name for tetrises in the game info table.
const GAME_INFO_TETRISES: &str = "tetrises";
/// Column name for tspins in the game info table.
const GAME_INFO_TSPINS: &str = "tspins";
/// Column name for tspin singles in the game info table.
const GAME_INFO_TSPINS_SINGLES: &str = "tspin_singles";
/// Column name for tspin doubles in the game info table.
const GAME_INFO_TSPINS_DOUBLES: &str = "tspin_doubles";
/// Column name for tspin triples in the game info table.
const GAME_INFO_TSPINS_TRIPLES: &str = "tspin_triples";
/// Column name for minitspins in the game info table.
const GAME_INFO_MINI_TSPINS: &str = "minitspins";
/// Column name for minitspin singles in the game info table.
const GAME_INFO_MINI_TSPINS_SINGLES: &str = "minitspin_singles";

/// Column name for time endured in the classic game info table.
const CLASSIC_TIME_ENDURED: &str = "time_endured";
/// Column name for points in the classic game info table.
const CLASSIC_POINTS: &str = "points";
/// Column name for level reached in the classic game info table.
const CLASSIC_LEVEL_REACHED: &str = "level_reached";
/// Column name for game info ID in the classic game info table.
const CLASSIC_GAME_INFO_ID: &str = "game_info_id";

/// Column name for game info ID in the blitz game info table.
const BLITZ_GAME_INFO_ID: &str = "game_info_id";
/// Column name for points in the blitz game info table.
const BLITZ_POINTS: &str = "points";

/// Column name for game info ID in the lines game info table.
const LINES_GAME_INFO_ID: &str = "game_info_id";
/// Column name for time endured in the lines game info table.
const LINES_TIME_ENDURED: &str = "time_endured";
