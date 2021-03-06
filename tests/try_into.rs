#![cfg(feature = "nightly")]
#![feature(try_from)]
#![allow(dead_code)]

#[macro_use]
extern crate derive_more;

use std::convert::{TryFrom, TryInto};

#[derive(Clone, Copy, TryInto)]
enum MixedInts {
    SmallInt(i32),
    NamedBigInt { int: i64 },
    TwoSmallInts(i32, i32),
    NamedBigInts { x: i64, y: i64 },
    Unsigned(u32),
    NamedUnsigned { x: u32 },
    Unit,
}

#[test]
fn test_try_into() {
    let i = MixedInts::SmallInt(42);
    assert_eq!(Ok(42i32), i.try_into());
    assert_eq!(
        i64::try_from(i),
        Err("Only NamedBigInt can be converted to i64")
    );
    assert_eq!(
        <(i32, i32)>::try_from(i),
        Err("Only TwoSmallInts can be converted to (i32, i32)")
    );
    assert_eq!(
        <(i64, i64)>::try_from(i),
        Err("Only NamedBigInts can be converted to (i64, i64)")
    );
    assert_eq!(
        u32::try_from(i),
        Err("Only Unsigned, NamedUnsigned can be converted to u32")
    );
    assert_eq!(<()>::try_from(i), Err("Only Unit can be converted to ()"));

    let i = MixedInts::NamedBigInt { int: 42 };
    assert_eq!(
        i32::try_from(i),
        Err("Only SmallInt can be converted to i32")
    );
    assert_eq!(Ok(42i64), i.try_into());
    assert_eq!(
        <(i32, i32)>::try_from(i),
        Err("Only TwoSmallInts can be converted to (i32, i32)")
    );
    assert_eq!(
        <(i64, i64)>::try_from(i),
        Err("Only NamedBigInts can be converted to (i64, i64)")
    );
    assert_eq!(
        u32::try_from(i),
        Err("Only Unsigned, NamedUnsigned can be converted to u32")
    );
    assert_eq!(<()>::try_from(i), Err("Only Unit can be converted to ()"));

    let i = MixedInts::TwoSmallInts(42, 64);
    assert_eq!(
        i32::try_from(i),
        Err("Only SmallInt can be converted to i32")
    );
    assert_eq!(
        i64::try_from(i),
        Err("Only NamedBigInt can be converted to i64")
    );
    assert_eq!(Ok((42i32, 64i32)), i.try_into());
    assert_eq!(
        <(i64, i64)>::try_from(i),
        Err("Only NamedBigInts can be converted to (i64, i64)")
    );
    assert_eq!(
        u32::try_from(i),
        Err("Only Unsigned, NamedUnsigned can be converted to u32")
    );
    assert_eq!(<()>::try_from(i), Err("Only Unit can be converted to ()"));

    let i = MixedInts::NamedBigInts { x: 42, y: 64 };
    assert_eq!(
        i32::try_from(i),
        Err("Only SmallInt can be converted to i32")
    );
    assert_eq!(
        i64::try_from(i),
        Err("Only NamedBigInt can be converted to i64")
    );
    assert_eq!(
        <(i32, i32)>::try_from(i),
        Err("Only TwoSmallInts can be converted to (i32, i32)")
    );
    assert_eq!(Ok((42i64, 64i64)), i.try_into());
    assert_eq!(
        u32::try_from(i),
        Err("Only Unsigned, NamedUnsigned can be converted to u32")
    );
    assert_eq!(<()>::try_from(i), Err("Only Unit can be converted to ()"));

    let i = MixedInts::Unsigned(42);
    assert_eq!(
        i32::try_from(i),
        Err("Only SmallInt can be converted to i32")
    );
    assert_eq!(
        i64::try_from(i),
        Err("Only NamedBigInt can be converted to i64")
    );
    assert_eq!(
        <(i32, i32)>::try_from(i),
        Err("Only TwoSmallInts can be converted to (i32, i32)")
    );
    assert_eq!(
        <(i64, i64)>::try_from(i),
        Err("Only NamedBigInts can be converted to (i64, i64)")
    );
    assert_eq!(Ok(42u32), i.try_into());
    assert_eq!(<()>::try_from(i), Err("Only Unit can be converted to ()"));

    let i = MixedInts::NamedUnsigned { x: 42 };
    assert_eq!(
        i32::try_from(i),
        Err("Only SmallInt can be converted to i32")
    );
    assert_eq!(
        i64::try_from(i),
        Err("Only NamedBigInt can be converted to i64")
    );
    assert_eq!(
        i64::try_from(i),
        Err("Only NamedBigInt can be converted to i64")
    );
    assert_eq!(
        <(i32, i32)>::try_from(i),
        Err("Only TwoSmallInts can be converted to (i32, i32)")
    );
    assert_eq!(
        <(i64, i64)>::try_from(i),
        Err("Only NamedBigInts can be converted to (i64, i64)")
    );
    assert_eq!(Ok(42u32), i.try_into());
    assert_eq!(<()>::try_from(i), Err("Only Unit can be converted to ()"));

    let i = MixedInts::Unit;
    assert_eq!(
        i32::try_from(i),
        Err("Only SmallInt can be converted to i32")
    );
    assert_eq!(
        i64::try_from(i),
        Err("Only NamedBigInt can be converted to i64")
    );
    assert_eq!(
        <(i32, i32)>::try_from(i),
        Err("Only TwoSmallInts can be converted to (i32, i32)")
    );
    assert_eq!(
        <(i64, i64)>::try_from(i),
        Err("Only NamedBigInts can be converted to (i64, i64)")
    );
    assert_eq!(
        u32::try_from(i),
        Err("Only Unsigned, NamedUnsigned can be converted to u32")
    );
    assert_eq!(Ok(()), i.try_into());
}
