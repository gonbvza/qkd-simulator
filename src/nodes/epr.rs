use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::node)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EprNode {
    pub id: i32,
    pub name: String,
    pub in_use: bool,
    pub measurements: i64,
    pub node_type: String,
}
