use diesel::prelude::*;

/// Represents an active QKD process, tracking its start time and accepted pairs.
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Process {
    pub id: i32,
    pub started_at: i64,
    pub accepted_pairs: i32,
    pub key: Option<String>,
}

impl Process {
    pub fn new(current_time: i64) -> Process {
        Process {
            id: 0,
            started_at: current_time,
            accepted_pairs: 0,
            key: None,
        }
    }

    pub fn is_complete(&self, expected_pairs: i32) -> bool {
        self.accepted_pairs >= expected_pairs
    }
}
