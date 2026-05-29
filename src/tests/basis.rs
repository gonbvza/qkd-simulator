use crate::models::basis::{Basis, DST_BASES, SRC_BASES};
use crate::models::entangled_pair::Side;

#[test]
fn try_from_and_into_roundtrip() {
    assert_eq!(Basis::try_from(1).unwrap(), Basis::Deg0);
    assert_eq!(Basis::try_from(4).unwrap(), Basis::Deg45);

    let v: i32 = Basis::Deg45.into();
    assert_eq!(v, 4);
}

#[test]
fn angle_values() {
    assert_eq!(Basis::Deg0.angle_deg(), 0.0);
    assert_eq!(Basis::DegNeg22_5.angle_deg(), -22.5);
    assert_eq!(Basis::Deg22_5.angle_deg(), 22.5);
    assert_eq!(Basis::Deg45.angle_deg(), 45.0);
    assert_eq!(Basis::Deg90.angle_deg(), 90.0);
}

#[test]
fn base_sets_and_random_choice() {
    // Check const arrays content
    assert_eq!(SRC_BASES.len(), 3);
    assert!(SRC_BASES.contains(&Basis::Deg0));
    assert!(DST_BASES.contains(&Basis::Deg22_5));

    // Random choice should return one of the allowed bases
    let s = Basis::get_random_basis(Side::Source);
    assert!(SRC_BASES.contains(&s));

    let d = Basis::get_random_basis(Side::Destination);
    assert!(DST_BASES.contains(&d));
}
