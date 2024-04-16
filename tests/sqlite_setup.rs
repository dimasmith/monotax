#[cfg(feature = "sqlite")]
mod sqlite_setup {

    use monotax::db;
    use rusqlite::Connection;

    #[test]
    fn create_database() {
        let mut conn = Connection::open_in_memory().unwrap();
        db::init::create_schema(&mut conn).unwrap();

        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap();
        let tables = stmt.query_map([], |row| row.get(0)).unwrap();
        let tables: Vec<String> = tables.map(|r| r.unwrap()).collect();
        assert_eq!(tables, vec!["incomes"]);
    }
}
