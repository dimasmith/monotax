use rusqlite::types::Value;

pub enum Criteria {
    And(Vec<Box<dyn Criterion>>),
    Or(Vec<Box<dyn Criterion>>),
}

pub trait Criterion {
    fn where_clause(&self) -> Option<String>;

    fn params(&self) -> Option<(&str, Value)>;
}

impl Criteria {
    pub fn where_clause(&self) -> String {
        let operator = match self {
            Criteria::And(_) => " AND ",
            Criteria::Or(_) => " OR ",
        };
        let crits = match self {
            Criteria::And(crits) => crits,
            Criteria::Or(crits) => crits,
        };
        crits
            .iter()
            .filter_map(|c| c.where_clause())
            .collect::<Vec<String>>()
            .join(operator)
    }

    pub fn params(&self) -> Vec<(&str, Value)> {
        let crits = match self {
            Criteria::And(crits) => crits,
            Criteria::Or(crits) => crits,
        };
        crits
            .iter()
            .filter_map(|c| c.params())
            .collect::<Vec<(&str, Value)>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_object_safe() {
        let _crits: Vec<Box<dyn Criterion>> = vec![
            Box::new(AgeCriterion::Older(30)),
            Box::new(NameCriterion::Name("John".to_string())),
        ];
    }

    #[test]
    fn where_clause_no_criterion() {
        let criteria = Criteria::And(vec![]);

        let where_clause = criteria.where_clause();

        assert_eq!(where_clause, "");
    }

    #[test]
    fn params_no_criterion() {
        let criteria = Criteria::And(vec![]);

        let params = criteria.params();

        assert_eq!(params, &[]);
    }

    #[test]
    fn where_clause_single_criterion() {
        let age_criterion = AgeCriterion::Older(30);
        let criteria = Criteria::And(vec![Box::new(age_criterion)]);

        let where_clause = criteria.where_clause();

        assert_eq!(where_clause, "age > :age");
    }

    #[test]
    fn params_single_criterion() {
        let age_criterion = AgeCriterion::Older(30);
        let criteria = Criteria::And(vec![Box::new(age_criterion)]);

        let params = criteria.params();

        assert_eq!(params, &[(":age", Value::Integer(30))]);
    }

    #[test]
    fn where_clause_single_missing_criterion() {
        let age_criterion = AgeCriterion::Any;
        let criteria = Criteria::And(vec![Box::new(age_criterion)]);

        let where_clause = criteria.where_clause();

        assert_eq!(where_clause, "");
    }

    #[test]
    fn params_single_missing_criterion() {
        let age_criterion = AgeCriterion::Any;
        let criteria = Criteria::And(vec![Box::new(age_criterion)]);

        let params = criteria.params();

        assert_eq!(params, &[]);
    }

    #[test]
    fn where_clause_multiple_criteria() {
        let age_criterion = AgeCriterion::Older(30);
        let name_criterion = NameCriterion::Name("John".to_string());
        let criteria = Criteria::And(vec![Box::new(age_criterion), Box::new(name_criterion)]);

        let where_clause = criteria.where_clause();

        assert_eq!(where_clause, "age > :age AND name = :name");
    }

    #[test]
    fn params_multiple_criteria() {
        let age_criterion = AgeCriterion::Older(30);
        let name_criterion = NameCriterion::Name("John".to_string());
        let criteria = Criteria::And(vec![Box::new(age_criterion), Box::new(name_criterion)]);

        let params = criteria.params();

        assert_eq!(
            params,
            &[
                (":age", Value::Integer(30)),
                (":name", Value::Text("John".to_string()))
            ]
        );
    }

    #[test]
    fn where_clause_multiple_criteria_with_missing() {
        let age_criterion = AgeCriterion::Older(30);
        let name_criterion = NameCriterion::Any;
        let criteria = Criteria::And(vec![Box::new(age_criterion), Box::new(name_criterion)]);

        let where_clause = criteria.where_clause();

        assert_eq!(where_clause, "age > :age");
    }

    #[test]
    fn params_multiple_criteria_with_missing() {
        let age_criterion = AgeCriterion::Older(30);
        let name_criterion = NameCriterion::Any;
        let criteria = Criteria::And(vec![Box::new(age_criterion), Box::new(name_criterion)]);

        let params = criteria.params();

        assert_eq!(params, &[(":age", Value::Integer(30))]);
    }

    enum AgeCriterion {
        Older(u32),
        Any,
    }

    enum NameCriterion {
        Name(String),
        Any,
    }

    impl Criterion for AgeCriterion {
        fn where_clause(&self) -> Option<String> {
            let col = "age";
            match self {
                AgeCriterion::Older(_) => Some(format!("{col} > :{col}")),
                AgeCriterion::Any => None,
            }
        }

        fn params(&self) -> Option<(&str, Value)> {
            let col = ":age";
            match self {
                AgeCriterion::Older(age) => Some((col, Value::Integer(*age as i64))),
                AgeCriterion::Any => None,
            }
        }
    }

    impl Criterion for NameCriterion {
        fn where_clause(&self) -> Option<String> {
            let col = "name";
            match self {
                NameCriterion::Name(_) => Some(format!("{col} = :{col}")),
                NameCriterion::Any => None,
            }
        }

        fn params(&self) -> Option<(&str, Value)> {
            let col = ":name";
            match self {
                NameCriterion::Name(name) => Some((col, Value::Text(name.to_string()))),
                NameCriterion::Any => None,
            }
        }
    }
}
