use diesel::{dsl, insert_into, prelude::*, select};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Process {
    id: i32,
    started_at: i64,
}

impl Process {
   pub fn new() -> Process {
        
   } 
}
