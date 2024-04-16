#[cfg(feature = "sqlite")]
mod sqlite_repository {
    use chrono::NaiveDateTime;
    use monotax::{
        db::{
            self,
            repository::{load_all_incomes, save_incomes},
        },
        income::Income,
    };
    use rusqlite::Connection;

    fn income(datetime: &str, amount: f64) -> Income {
        let datetime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").unwrap();
        Income::new(datetime, amount)
    }

    #[test]
    fn save_and_load_incomes() {
        let mut conn = Connection::open_in_memory().unwrap();
        db::init::create_schema(&mut conn).unwrap();

        let income1 = income("2024-04-13 14:00:00", 225.0);
        let income2 = income("2024-07-13 14:00:00", 325.0);
        let incomes = vec![income1.clone(), income2.clone()];

        let updated = save_incomes(&mut conn, &incomes).unwrap();

        let incomes = load_all_incomes(&mut conn).unwrap();
        assert_eq!(incomes.len(), 2, "unexpected number of incomes");
        assert_eq!(updated, 2, "unexpected number of updated rows");
        // incomes must be ordered by date
        assert_eq!(incomes[0], income1);
        assert_eq!(incomes[1], income2);
    }

    #[test]
    fn ignore_duplicate_incomes() {
        let mut conn = Connection::open_in_memory().unwrap();
        db::init::create_schema(&mut conn).unwrap();

        let income1 = income("2024-04-13 14:00:00", 225.0);
        let income2 = income("2024-07-13 14:00:00", 325.0);
        let income2_dup = income2.clone();
        let income1_dup = income1.clone();
        let incomes = vec![income1.clone(), income2.clone(), income2_dup, income1_dup];

        let updated = save_incomes(&mut conn, &incomes).unwrap();

        let incomes = load_all_incomes(&mut conn).unwrap();
        // duplicates must be ignored
        // so we expect only 2 unique incomes
        assert_eq!(updated, 2, "unexpected number of updated rows");
        assert_eq!(incomes.len(), 2, "unexpected number of incomes");
        // incomes must be ordered by date
        assert_eq!(incomes[0], income1);
        assert_eq!(incomes[1], income2);
    }
}
