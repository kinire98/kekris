use tokio::sync::OnceCell;

pub mod migrations;

pub mod store_game_info;

pub mod retreive_game_info;

static DB_URL: OnceCell<String> = OnceCell::const_new();

const GAME_TABLE_NAME: &str = "games";
const CLASSIC_TABLE_NAME: &str = "classic";
const BLITZ_TABLE_NAME: &str = "blitz";
const LINES_TABLE_NAME: &str = "lines";
const GAME_INFO_TABLE_NAME: &str = "game_info";

const GAME_INFO_PIECE_MOVES: &str = "piece_moves";
const GAME_INFO_SPINS: &str = "spins";
const GAME_INFO_LINES_CLEARED: &str = "lines_cleared";
const GAME_INFO_PIECES_USED: &str = "pieces_used";
const GAME_INFO_SINGLES: &str = "singles";
const GAME_INFO_DOUBLES: &str = "doubles";
const GAME_INFO_TRIPLES: &str = "triples";
const GAME_INFO_TETRISES: &str = "tetrises";
const GAME_INFO_TSPINS: &str = "tspins";
const GAME_INFO_TSPINS_SINGLES: &str = "tspins_singles";
const GAME_INFO_TSPINS_DOUBLES: &str = "tspins_doubles";
const GAME_INFO_TSPINS_TRIPLES: &str = "tspins_triples";
const GAME_INFO_MINI_TSPINS: &str = "minitspins";
const GAME_INFO_MINI_TSPINS_SINGLES: &str = "minitspins_singles";

const CLASSIC_TIME_ENDURED: &str = "time_endured";
const CLASSIC_POINTS: &str = "points";
const CLASSIC_LEVEL_REACHED: &str = "level_reached";
const CLASSIC_GAME_INFO_ID: &str = "game_info_id";

const BLITZ_GAME_INFO_ID: &str = "game_info_id";
const BLITZ_POINTS: &str = "points";

const LINES_GAME_INFO_ID: &str = "game_info_id";
const LINES_TIME_ENDURED: &str = "time_endured";
