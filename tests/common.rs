use sqlx::{migrate, SqlitePool};
use uuid::Uuid;

pub async fn connect_to_test_db() -> SqlitePool {
    let db_name = Uuid::new_v4().to_string();
    let db_file = format!("file:{}", db_name);
    let database_url = format!("{}?mode=memory&cache=shared", &db_file);
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("sqlx connection failed");

    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("sqlx migration failed");

    pool
}
