use crate::fr;
use crate::MyFraction;

macro_rules! assert_print {
    ($left:expr, $op:tt, $right:expr,$($type:ty),*) => {
        assert!($left $op $right, "{} {} {}", $left, stringify!($op), $right);
    };
}

#[test]
fn test() {
    assert_eq!(fr!(10, 2), fr!(5, 1));
    assert_print!(fr!(152541421414141124124124, 152541421414141124124124),==,1,
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

    assert_print!(fr!(10, 2),==,fr!(10, 2),
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
    assert_print!(fr!(10, 2),>,fr!(10, 3),
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
    assert_print!(fr!(10, 3),<,fr!(10, 2),
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

    assert_print!(fr!(10, 2),==,5,
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
    assert_print!(fr!(10, 2),>,4,
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
    assert_print!(fr!(10, 2),<,6,
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

    assert_print!(5,==,fr!(10,2),
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
    assert_print!(2,<,fr!(10, 3),
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
    assert_print!(6,>,fr!(10, 3),
        usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

    assert_eq!(fr!(7, 2).as_mixed(), "3(1/2)".to_string());
    assert_eq!(fr!(6, 2).as_mixed(), "3".to_string());
    assert_eq!(fr!(1, 6).as_mixed(), "(1/6)".to_string());

    assert_eq!(fr!(1, 3).as_f64(), 1. / 3.);

    println!("{}", fr!(21231, 421).as_mixed());
}
