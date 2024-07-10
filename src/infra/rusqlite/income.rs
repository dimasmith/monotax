use async_trait::async_trait;
use monotax::db::repository::{load_all_incomes, save_incomes};
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

    async fn find_all(&mut self) -> anyhow::Result<Vec<Income>> {
        let conn = &mut self.conn;
        block_in_place(move || load_all_incomes(conn))
    }
}
