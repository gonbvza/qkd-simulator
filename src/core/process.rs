use std::sync::Arc;

use diesel::prelude::*;

use crate::{core::event_loop::EventLoop, error::ProcessError, establish_connection};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Process {
    pub id: i32,
    pub started_at: i64,
    pub accepted_pairs: i32,
}

impl Process {
    pub fn new(current_time: i64) -> Process {
        Process {
            id: 0,
            started_at: current_time,
            accepted_pairs: 0,
        }
    }
}
