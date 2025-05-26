use std::path::PathBuf;

use sqlx::{SqlitePool, migrate::Migrator};
use tauri::{AppHandle, Manager};
use tokio::fs::{self, File};

static MIGRATOR: Migrator = sqlx::migrate!();
/// Runs the database migrations.
///
/// This function checks if the database file exists, creates it if it doesn't,
/// and then runs the migrations to ensure the database schema is up-to-date.
///
/// # Arguments
///
/// * `app` - A Tauri `AppHandle` for accessing the application's data directory.
pub async fn run_migrations(app: AppHandle) {
    let base: PathBuf = app.path().app_data_dir().unwrap();
    let db_path = base.join("kekris.db");
    if !db_path.exists() {
        fs::create_dir_all(&base).await.unwrap();
        File::create_new(&db_path).await.unwrap();
    }
    let url = format!("sqlite://{}", db_path.display());

    super::DB_URL.set(url.clone()).unwrap();

    let db = SqlitePool::connect(&url).await.expect("DB failed");

    // Run migrations
    MIGRATOR.run(&db).await.expect("Migration failed");
}
