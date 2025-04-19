use sqlx::{Pool, Sqlite, SqlitePool};

use crate::game::game_info::{BlitzGameInfo, ClassicGameInfo, GameInfo, LinesGameInfo};

pub async fn store_game_info(info: GameInfo) {
    let Some(url) = super::DB_URL.get() else {
        panic!("DB_URL Not set")
    };
    let pool = SqlitePool::connect(url).await.unwrap();

    let result = sqlx::query(
        r#"
        INSERT INTO game_info (
            piece_moves, spins, lines_cleared, pieces_used,
            singles, doubles, triples, tetrises, tspins,
            tspin_singles, tspin_doubles, tspin_triples,
            minitspins, minitspin_singles
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(info.piece_moves())
    .bind(info.spins())
    .bind(info.lines_cleared())
    .bind(info.pieces_used())
    .bind(info.singles())
    .bind(info.doubles())
    .bind(info.triples())
    .bind(info.tetrises())
    .bind(info.tspins())
    .bind(info.tspin_singles())
    .bind(info.tspin_doubles())
    .bind(info.tspin_triples())
    .bind(info.minitspins())
    .bind(info.minitspin_singles())
    .execute(&pool)
    .await
    .unwrap();
    let type_of_game = match info.type_of_info() {
        crate::game::game_info::GameTypeInfo::Classic(type_info) => (
            store_classic(&pool, type_info, result.last_insert_rowid()).await,
            "classic",
        ),
        crate::game::game_info::GameTypeInfo::Lines(type_info) => (
            store_lines(&pool, type_info, result.last_insert_rowid()).await,
            "lines",
        ),
        crate::game::game_info::GameTypeInfo::Blitz(type_info) => (
            store_blitz(&pool, type_info, result.last_insert_rowid()).await,
            "blitz",
        ),
    };
    sqlx::query(
        r#"
    INSERT INTO games (game_type, id_game)
    VALUES (?1, ?2)
    "#,
    )
    .bind(type_of_game.1)
    .bind(type_of_game.0)
    .execute(&pool)
    .await
    .unwrap();
}

async fn store_classic(pool: &Pool<Sqlite>, type_info: ClassicGameInfo, id: i64) -> i64 {
    sqlx::query(
        r#"
            INSERT INTO classic (time_endured, points, level_reached, game_info_id)
            VALUES (?1, ?2, ?3, ?4)
            "#,
    )
    .bind(type_info.time_endured())
    .bind(type_info.points())
    .bind(type_info.level_reached())
    .bind(id)
    .execute(pool)
    .await
    .unwrap()
    .last_insert_rowid()
}
async fn store_lines(pool: &Pool<Sqlite>, type_info: LinesGameInfo, id: i64) -> i64 {
    sqlx::query(
        r#"
            INSERT INTO lines (time_endured, game_info_id)
            VALUES (?1, ?2)
            "#,
    )
    .bind(type_info.time_endured())
    .bind(id)
    .execute(pool)
    .await
    .unwrap()
    .last_insert_rowid()
}
async fn store_blitz(pool: &Pool<Sqlite>, type_info: BlitzGameInfo, id: i64) -> i64 {
    sqlx::query(
        r#"
            INSERT INTO blitz (points, game_info_id)
            VALUES (?1, ?2)
            "#,
    )
    .bind(type_info.points())
    .bind(id)
    .execute(pool)
    .await
    .unwrap()
    .last_insert_rowid()
}
