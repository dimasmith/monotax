use monotax::db;
use rusqlite::Connection;

pub async fn connect_to_test_db() -> Connection {
    let mut conn = Connection::open_in_memory().unwrap();
    db::init::create_schema(&mut conn).unwrap();
    conn
}
