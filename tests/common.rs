use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;
use sqlx::{migrate, pool, Connection, SqliteConnection, SqlitePool};
use uuid::Uuid;

pub async fn connect_to_test_db() -> Pool<SqliteConnectionManager> {
    let db_name = Uuid::new_v4().to_string();
    let db_file = format!("file:{}", db_name);
    let database_url = format!("{}?mode=memory&cache=shared", &db_file);
    let mut xconn = SqliteConnection::connect(&database_url)
        .await
        .expect("sqlx connection failed");

    migrate!("./migrations")
        .run(&mut xconn)
        .await
        .expect("sqlx migration failed");

    r2d2::Pool::builder()
        .max_size(1)
        .build(
            r2d2_sqlite::SqliteConnectionManager::file(db_file).with_flags(
                OpenFlags::SQLITE_OPEN_READ_WRITE
                    | OpenFlags::SQLITE_OPEN_SHARED_CACHE
                    | OpenFlags::SQLITE_OPEN_MEMORY,
            ),
        )
        .unwrap()

    // rusqlite::Connection::open_with_flags(
    //     db_file,
    //     OpenFlags::SQLITE_OPEN_READ_WRITE
    //         | OpenFlags::SQLITE_OPEN_SHARED_CACHE
    //         | OpenFlags::SQLITE_OPEN_MEMORY,
    // )
    // .unwrap()
}

pub async fn connect_to_test_db_sqlx() -> SqlitePool {
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
