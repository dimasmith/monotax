use async_trait::async_trait;
use monotax::db::repository::save_incomes;
use monotax::db::IncomeRepository;
use monotax::domain::income::Income;
use rusqlite::Connection;
use tokio::task::block_in_place;

pub struct RusqliteIncomeRepository {
    conn: Connection,
}

impl RusqliteIncomeRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl IncomeRepository for RusqliteIncomeRepository {
    async fn save_all(&mut self, incomes: &[Income]) -> anyhow::Result<usize> {
        let conn = &mut self.conn;
        block_in_place(move || save_incomes(conn, incomes))
    }
}
