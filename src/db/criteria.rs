use rusqlite::types::Value;

use crate::income::criteria::IncomeCriteria;

pub trait SqlCriterion {
    fn where_clause(&self) -> Option<String>;

    fn params(&self) -> Option<(&str, Value)>;
}

pub trait SqlCriteria {
    fn where_clause(&self) -> String;

    fn params(&self) -> Vec<(&str, Value)>;
}

impl SqlCriteria for IncomeCriteria {
    fn where_clause(&self) -> String {
        let operator = " AND ";
        let crits = self.criteria();
        crits
            .iter()
            .filter_map(|c| c.where_clause())
            .collect::<Vec<String>>()
            .join(operator)
    }

    fn params(&self) -> Vec<(&str, Value)> {
        let crits = self.criteria();
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
    fn where_clause_no_criterion() {
        let empty_criteria = IncomeCriteria::new(&[]);

        let where_clause = empty_criteria.where_clause();

        assert_eq!(where_clause, "");
    }

    #[test]
    fn params_no_criterion() {
        let empty_criteria = IncomeCriteria::new(&[]);

        let params = empty_criteria.params();

        assert_eq!(params, &[]);
    }
}
