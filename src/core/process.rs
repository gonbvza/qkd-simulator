use std::sync::Arc;

use diesel::prelude::*;

use crate::{
    core::event_loop::EventLoop, database::process::create_new_process, error::ProcessError,
    establish_connection,
};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Process {
    pub id: i32,
    pub started_at: i64,
}

impl Process {
    pub fn new() -> Result<Process, ProcessError> {
        let mut conn = establish_connection();
        let current_time = {
            let loop_pair = Arc::clone(&*EventLoop::instance());
            let (event_loop, _) = &*loop_pair;

            let mut guard = event_loop.lock().unwrap();
            guard.get_current_time()
        };
        create_new_process(&mut conn, current_time)
    }
}
