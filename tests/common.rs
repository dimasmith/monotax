use rusqlite::OpenFlags;
use sqlx::{migrate, Connection, SqliteConnection};
use uuid::Uuid;

pub async fn connect_to_test_db() -> rusqlite::Connection {
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

    rusqlite::Connection::open_with_flags(
        db_file,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_SHARED_CACHE
            | OpenFlags::SQLITE_OPEN_MEMORY,
    )
    .unwrap()
}
