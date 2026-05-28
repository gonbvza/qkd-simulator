use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::Integer,
};

use crate::models::entangled_pair::Side;

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

    pub fn src_bases() -> &'static [Basis; 3] {
        &SRC_BASES
    }

    pub fn dst_bases() -> &'static [Basis; 3] {
        &DST_BASES
    }

    pub fn get_random_basis(side: Side) -> Basis {
        let random_number = rand::random_range(0..3);
        match side {
            Side::Source => Basis::src_bases()[random_number],
            Side::Destination => Basis::dst_bases()[random_number],
        }
    }
}

pub const SRC_BASES: [Basis; 3] = [Basis::Deg0, Basis::Deg45, Basis::Deg90];
pub const DST_BASES: [Basis; 3] = [Basis::DegNeg22_5, Basis::Deg22_5, Basis::Deg45];

impl TryFrom<i32> for Basis {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Basis::Deg0),
            2 => Ok(Basis::DegNeg22_5),
            3 => Ok(Basis::Deg22_5),
            4 => Ok(Basis::Deg45),
            5 => Ok(Basis::Deg90),
            v => Err(format!("Unknown Basis value: {}", v)),
        }
    }
}

impl From<Basis> for i32 {
    fn from(b: Basis) -> Self {
        match b {
            Basis::Deg0 => 1,
            Basis::DegNeg22_5 => 2,
            Basis::Deg22_5 => 3,
            Basis::Deg45 => 4,
            Basis::Deg90 => 5,
        }
    }
}
