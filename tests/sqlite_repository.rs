#[cfg(feature = "sqlite")]
mod sqlite_repository {
    use chrono::NaiveDateTime;
    use monotax::{
        db::{
            self,
            repository::{find_incomes, load_all_incomes, save_incomes},
        },
        income::{
            criteria::{IncomeCriteria, IncomeCriterion, QuarterFilter, YearFilter},
            Income,
        },
        time::Quarter,
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

    #[test]
    fn filter_incomes_on_quarters() {
        let mut conn = Connection::open_in_memory().unwrap();
        db::init::create_schema(&mut conn).unwrap();

        let q1_2024 = income("2024-01-13 14:00:00", 125.0);
        let q2_2024 = income("2024-04-13 14:00:00", 225.0);
        let q3_2024 = income("2024-07-13 14:00:00", 325.0);
        let q4_2024 = income("2024-10-13 14:00:00", 425.0);
        let incomes = vec![
            q1_2024.clone(),
            q2_2024.clone(),
            q3_2024.clone(),
            q4_2024.clone(),
        ];

        let _ = save_incomes(&mut conn, &incomes).unwrap();

        let q3_only = QuarterFilter::Only(Quarter::Q3);
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[IncomeCriterion::Quarter(q3_only)]),
        )
        .unwrap();
        assert_eq!(filtered_incomes, vec![q3_2024.clone()]);

        let q2_ytd = QuarterFilter::Ytd(Quarter::Q2);
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[IncomeCriterion::Quarter(q2_ytd)]),
        )
        .unwrap();
        assert_eq!(filtered_incomes, vec![q1_2024.clone(), q2_2024.clone()]);

        let q_any = QuarterFilter::Any;
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[IncomeCriterion::Quarter(q_any)]),
        )
        .unwrap();
        assert_eq!(
            filtered_incomes,
            vec![
                q1_2024.clone(),
                q2_2024.clone(),
                q3_2024.clone(),
                q4_2024.clone()
            ]
        );
    }

    #[test]
    fn filter_incomes_on_years() {
        let mut conn = Connection::open_in_memory().unwrap();
        db::init::create_schema(&mut conn).unwrap();

        let y2023 = income("2023-01-13 14:00:00", 125.0);
        let y2024 = income("2024-01-13 14:00:00", 225.0);
        let y2025 = income("2025-01-13 14:00:00", 325.0);
        let incomes = vec![y2023.clone(), y2024.clone(), y2025.clone()];

        let _ = save_incomes(&mut conn, &incomes).unwrap();

        let y2024_only = YearFilter::One(2024);
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[IncomeCriterion::from(y2024_only)]),
        )
        .unwrap();
        assert_eq!(filtered_incomes, vec![y2024.clone()]);

        let y_any = YearFilter::Any;
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[IncomeCriterion::from(y_any)]),
        )
        .unwrap();
        assert_eq!(
            filtered_incomes,
            vec![y2023.clone(), y2024.clone(), y2025.clone()]
        );
    }

    #[test]
    fn filter_incomes_on_quarters_and_years() {
        let mut conn = Connection::open_in_memory().unwrap();
        db::init::create_schema(&mut conn).unwrap();

        let q1_2023 = income("2023-01-13 14:00:00", 125.0);
        let q2_2023 = income("2023-04-13 14:00:00", 225.0);
        let q3_2023 = income("2023-07-13 14:00:00", 325.0);
        let q4_2023 = income("2023-10-13 14:00:00", 425.0);
        let q1_2024 = income("2024-01-13 14:00:00", 125.0);
        let q2_2024 = income("2024-04-13 14:00:00", 225.0);
        let q3_2024 = income("2024-07-13 14:00:00", 325.0);
        let q4_2024 = income("2024-10-13 14:00:00", 425.0);
        let incomes = vec![
            q1_2023.clone(),
            q2_2023.clone(),
            q3_2023.clone(),
            q4_2023.clone(),
            q1_2024.clone(),
            q2_2024.clone(),
            q3_2024.clone(),
            q4_2024.clone(),
        ];

        let _ = save_incomes(&mut conn, &incomes).unwrap();

        let q3_2024_only = QuarterFilter::Only(Quarter::Q3);
        let y2024_only = YearFilter::One(2024);
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[
                IncomeCriterion::from(q3_2024_only),
                IncomeCriterion::from(y2024_only),
            ]),
        )
        .unwrap();
        assert_eq!(filtered_incomes, vec![q3_2024.clone()]);

        // test for ytd q3 in 2023
        let q3_ytd_2023 = QuarterFilter::Ytd(Quarter::Q3);
        let y2023_only = YearFilter::One(2023);
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[
                IncomeCriterion::from(q3_ytd_2023),
                IncomeCriterion::from(y2023_only),
            ]),
        );
        assert_eq!(
            filtered_incomes.unwrap(),
            vec![q1_2023.clone(), q2_2023.clone(), q3_2023.clone()]
        );

        // test for q4 in any year
        let q4_only = QuarterFilter::Only(Quarter::Q4);
        let y_any = YearFilter::Any;
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[IncomeCriterion::from(q4_only), IncomeCriterion::from(y_any)]),
        );
        assert_eq!(
            filtered_incomes.unwrap(),
            vec![q4_2023.clone(), q4_2024.clone()]
        );

        // test for whole year 2024 regardless of quarter
        let y2024_only = YearFilter::One(2024);
        let q_ay = QuarterFilter::Any;
        let filtered_incomes = find_incomes(
            &mut conn,
            IncomeCriteria::new(&[
                IncomeCriterion::from(q_ay),
                IncomeCriterion::from(y2024_only),
            ]),
        );
        assert_eq!(
            filtered_incomes.unwrap(),
            vec![
                q1_2024.clone(),
                q2_2024.clone(),
                q3_2024.clone(),
                q4_2024.clone()
            ]
        );
    }
}
