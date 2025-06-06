use crate::models::game_info::{BlitzGameInfo, ClassicGameInfo, GameTypeInfo, LinesGameInfo};
use crate::models::{
    emit_game_info::EmitGameInfo, game_generic_info::GameGenericInfo, game_info::GameInfo,
};
use sqlx::Row;
use sqlx::{Pool, Sqlite, SqlitePool};

/// Retrieves the last game's information and all results for that game type.
pub async fn retreive_last_games_info() -> EmitGameInfo {
    let Some(url) = super::DB_URL.get() else {
        panic!("DB_URL Not set")
    };
    let pool = SqlitePool::connect(url).await.unwrap();
    let generic_info = sqlx::query_as::<_, GameGenericInfo>(&format!(
        r#"
        SELECT id, game_type, id_game
        FROM {}
        ORDER BY id DESC 
        LIMIT 1
        "#,
        super::GAME_TABLE_NAME
    ))
    .fetch_one(&pool)
    .await
    .unwrap();
    EmitGameInfo::new(
        get_last_result(generic_info.game_id(), generic_info.game_type(), &pool).await,
        get_all_results(generic_info.game_type(), &pool).await,
    )
}
/// Retrieves the last classic game's information.
pub async fn retreive_last_classic_info() -> EmitGameInfo {
    get_all_info(super::CLASSIC_TABLE_NAME).await
}
/// Retrieves the last lines game's information.
pub async fn retreive_last_lines_info() -> EmitGameInfo {
    get_all_info(super::LINES_TABLE_NAME).await
}
/// Retrieves the last blitz game's information.
pub async fn retreive_last_blitz_info() -> EmitGameInfo {
    get_all_info(super::BLITZ_TABLE_NAME).await
}
/// Retrieves all information for a specific game type.
async fn get_all_info(type_of_game: &str) -> EmitGameInfo {
    let Some(url) = super::DB_URL.get() else {
        panic!("DB_URL Not set")
    };
    let pool = SqlitePool::connect(url).await.unwrap();
    let generic_info_result = sqlx::query_as::<_, GameGenericInfo>(&format!(
        r#"
        SELECT *
        FROM {} 
        WHERE game_type =  ?1
        ORDER BY id DESC 
        LIMIT 1 
        "#,
        super::GAME_TABLE_NAME
    ))
    .bind(type_of_game)
    .fetch_one(&pool)
    .await;
    let Ok(generic_info) = generic_info_result else {
        return EmitGameInfo::empty();
    };
    EmitGameInfo::new(
        get_last_result(generic_info.game_id(), type_of_game, &pool).await,
        get_all_results(type_of_game, &pool).await,
    )
}
/// Retrieves all results for a specific game type.
async fn get_all_results(type_of_game: &str, pool: &Pool<Sqlite>) -> Vec<GameInfo> {
    let all_of_type = sqlx::query(
        r#"
        SELECT id_game
        FROM games
        WHERE game_type = ?1
        "#,
    )
    .bind(type_of_game)
    .fetch_all(pool)
    .await
    .unwrap();
    let mut vec_of_results = vec![];
    for el in all_of_type {
        vec_of_results.push(get_last_result(el.get("id_game"), type_of_game, pool).await);
    }
    vec_of_results
}
/// Retrieves the last result for a specific game.
async fn get_last_result(id: u32, type_of_game: &str, pool: &Pool<Sqlite>) -> GameInfo {
    let specific_info = if type_of_game == super::CLASSIC_TABLE_NAME {
        get_classic(id, pool).await
    } else if type_of_game == super::LINES_TABLE_NAME {
        get_lines(id, pool).await
    } else {
        get_blitz(id, pool).await
    };
    let common_info = sqlx::query(&format!(
        r#"SELECT * FROM {} WHERE id = ?1"#,
        super::GAME_INFO_TABLE_NAME
    ))
    .bind(specific_info.1)
    .fetch_one(pool)
    .await
    .unwrap();
    GameInfo::new_from(
        common_info.get(super::GAME_INFO_PIECE_MOVES),
        common_info.get(super::GAME_INFO_SPINS),
        common_info.get(super::GAME_INFO_LINES_CLEARED),
        common_info.get(super::GAME_INFO_PIECES_USED),
        common_info.get(super::GAME_INFO_SINGLES),
        common_info.get(super::GAME_INFO_DOUBLES),
        common_info.get(super::GAME_INFO_TRIPLES),
        common_info.get(super::GAME_INFO_TETRISES),
        common_info.get(super::GAME_INFO_TSPINS),
        common_info.get(super::GAME_INFO_TSPINS_SINGLES),
        common_info.get(super::GAME_INFO_TSPINS_DOUBLES),
        common_info.get(super::GAME_INFO_TSPINS_TRIPLES),
        common_info.get(super::GAME_INFO_MINI_TSPINS),
        common_info.get(super::GAME_INFO_MINI_TSPINS_SINGLES),
        specific_info.0,
    )
}
/// Retrieves the classic game information.
async fn get_classic(id: u32, pool: &Pool<Sqlite>) -> (GameTypeInfo, i64) {
    let info = sqlx::query(&format!(
        r#"SELECT * FROM {} WHERE id = ?1"#,
        super::CLASSIC_TABLE_NAME
    ))
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    (
        GameTypeInfo::Classic(ClassicGameInfo::new(
            info.get(super::CLASSIC_TIME_ENDURED),
            info.get(super::CLASSIC_POINTS),
            info.get(super::CLASSIC_LEVEL_REACHED),
        )),
        info.get(super::CLASSIC_GAME_INFO_ID),
    )
}
/// Retrieves the lines game information.
async fn get_lines(id: u32, pool: &Pool<Sqlite>) -> (GameTypeInfo, i64) {
    let info = sqlx::query(&format!(
        r#"SELECT * FROM {} WHERE id = ?1"#,
        super::LINES_TABLE_NAME
    ))
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    (
        GameTypeInfo::Lines(LinesGameInfo::new(info.get(super::LINES_TIME_ENDURED))),
        info.get(super::LINES_GAME_INFO_ID),
    )
}
/// Retrieves the blitz game information.
async fn get_blitz(id: u32, pool: &Pool<Sqlite>) -> (GameTypeInfo, i64) {
    let info = sqlx::query(&format!(
        r#"SELECT * FROM {} WHERE id = ?1"#,
        super::BLITZ_TABLE_NAME
    ))
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    (
        GameTypeInfo::Blitz(BlitzGameInfo::new(info.get(super::BLITZ_POINTS))),
        info.get(super::BLITZ_GAME_INFO_ID),
    )
}
