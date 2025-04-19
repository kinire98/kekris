use tokio::sync::OnceCell;

pub mod migrations;

pub mod store_game_info;

static DB_URL: OnceCell<String> = OnceCell::const_new();
