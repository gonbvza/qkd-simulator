use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    prelude::Queryable,
    serialize::{self, Output, ToSql},
    sql_types::Integer,
    Selectable,
};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Integer)]
pub enum Basis {
    Deg0 = 1,
    DegNeg22_5 = 2,
    Deg22_5 = 3,
    Deg45 = 4,
    Deg90 = 5,
}

// Implement FromSql (single value deserialization) instead of FromSqlRow
impl FromSql<Integer, Pg> for Basis {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Self::Deg0),
            2 => Ok(Self::DegNeg22_5),
            3 => Ok(Self::Deg22_5),
            4 => Ok(Self::Deg45),
            5 => Ok(Self::Deg90),
            v => Err(format!("Unknown value {} for Basis", v).into()),
        }
    }
}

impl ToSql<Integer, Pg> for Basis {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            Basis::Deg0 => <i32 as ToSql<Integer, Pg>>::to_sql(&1, out),
            Basis::DegNeg22_5 => <i32 as ToSql<Integer, Pg>>::to_sql(&2, out),
            Basis::Deg22_5 => <i32 as ToSql<Integer, Pg>>::to_sql(&3, out),
            Basis::Deg45 => <i32 as ToSql<Integer, Pg>>::to_sql(&4, out),
            Basis::Deg90 => <i32 as ToSql<Integer, Pg>>::to_sql(&5, out),
        }
    }
}

impl Basis {
    pub fn angle_deg(&self) -> f64 {
        match self {
            Basis::Deg0 => 0.0,
            Basis::DegNeg22_5 => -22.5,
            Basis::Deg22_5 => 22.5,
            Basis::Deg45 => 45.0,
            Basis::Deg90 => 90.0,
        }
    }

    pub fn left_bases() -> [Basis; 3] {
        [Basis::Deg0, Basis::Deg45, Basis::Deg90]
    }

    pub fn right_bases() -> [Basis; 3] {
        [Basis::DegNeg22_5, Basis::Deg22_5, Basis::Deg45]
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::measurements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    pub id: i32,
    pub node_id: i32,
    pub basis: Basis,
    pub measurement_id: i64,
    pub value: i16,
    pub consumed: bool,
}
