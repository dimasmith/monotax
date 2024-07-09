use monotax::db::repository::save_incomes;
use monotax::db::IncomeRepository;
use monotax::domain::income::Income;
use rusqlite::Connection;

pub struct RusqliteIncomeRepository {
    conn: Connection,
}

impl RusqliteIncomeRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

impl IncomeRepository for RusqliteIncomeRepository {
    fn save_all(&mut self, incomes: &[Income]) -> anyhow::Result<usize> {
        let conn = &mut self.conn;
        save_incomes(conn, incomes)
    }
}
