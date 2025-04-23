use std::path::PathBuf;

use sqlx::{SqlitePool, migrate::Migrator};
use tauri::{AppHandle, Manager};
use tokio::fs::File;

static MIGRATOR: Migrator = sqlx::migrate!();
pub async fn run_migrations(app: AppHandle) {
    let base: PathBuf = app.path().app_data_dir().unwrap();
    let db_path = base.join("kekris.db");
    if !db_path.exists() {
        File::create(base).await.unwrap();
        File::create(&db_path).await.unwrap();
    }
    let url = format!("sqlite://{}", db_path.display());

    super::DB_URL.set(url.clone()).unwrap();

    let db = SqlitePool::connect(&url).await.expect("DB failed");

    // Run migrations
    MIGRATOR.run(&db).await.expect("Migration failed");
}
