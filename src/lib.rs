use std::cmp::PartialEq;
use std::cmp::{self, Ordering};
use std::convert::From;
use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(test)]
mod test;

#[macro_export]
macro_rules! fr {
    ($num:expr,$den:expr) => {
        MyFraction::new($num, $den)
    };
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct MyFraction {
    pub numerator: i128,
    pub denominator: i128,
}
impl MyFraction {
    pub fn new(num: i128, den: i128) -> Self {
        assert_ne!(den, 0);
        Self {
            numerator: num,
            denominator: den,
        }
        .simplify()
    }
    fn simplify(mut self) -> Self {
        let abs_num = self.numerator.abs();
        let abs_den = self.denominator.abs();
        let mut max_factor: i128 = 1;

        if abs_num % abs_den == 0 {
            max_factor = abs_den;
        } else {
            for i in 1..(cmp::max(abs_num, abs_den) / 2) {
                let num_is: bool = abs_num % i == 0;
                let den_is: bool = abs_den % i == 0;
                if num_is && den_is {
                    max_factor = i;
                } else if !(num_is && den_is) {
                } else {
                    break;
                }
            }
        }
        if self.denominator.is_negative() {
            self.numerator *= -1;
            self.denominator *= -1;
        }
        self.numerator /= max_factor;
        self.denominator /= max_factor;
        assert_ne!(self.denominator, 0);
        self
    }
    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
    pub fn as_mixed(&self) -> String {
        let rem = self.numerator % self.denominator;
        if self.denominator == 1 {
            return format!("{}", self.numerator);
        }
        if self.numerator < self.denominator {
            return format!("({}/{})", self.numerator, self.denominator);
        }
        let whole_part = (self.numerator - rem) / self.denominator;
        format!("{whole_part}({rem}/{})", self.denominator)
    }
}

//==========================================================//

impl Sub<MyFraction> for MyFraction {
    type Output = MyFraction;

    fn sub(self, rhs: MyFraction) -> Self::Output {
        if self.denominator == rhs.denominator {
            return Self {
                numerator: self.numerator - rhs.numerator,
                denominator: self.denominator,
            }
            .simplify();
        } else {
            Self {
                numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
                denominator: self.denominator * rhs.denominator,
            }
            .simplify()
        }
    }
}
macro_rules! implSub {
    ($($type:ty),*) => { $(
        impl Sub<$type> for MyFraction {
            type Output = MyFraction;

            fn sub(self, rhs: $type) -> Self::Output {
                Self {
                    numerator: self.numerator - rhs as i128 * self.denominator,
                    denominator: self.denominator,
                }
                .simplify()
            }
        }
        impl Sub<MyFraction> for $type {
            type Output = MyFraction;

            fn sub(self, rhs: MyFraction) -> Self::Output {
                MyFraction {
                    numerator: self as i128 * rhs.denominator - rhs.numerator,
                    denominator: rhs.denominator,
                }
                .simplify()
            }
        }
        )*
    };
}
implSub!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl Add<MyFraction> for MyFraction {
    type Output = MyFraction;

    fn add(self, rhs: MyFraction) -> Self::Output {
        if self.denominator == rhs.denominator {
            return Self {
                numerator: self.numerator + rhs.numerator,
                denominator: self.denominator,
            }
            .simplify();
        } else {
            Self {
                numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                denominator: self.denominator * rhs.denominator,
            }
            .simplify()
        }
    }
}
macro_rules! implAdd {
    ($($type:ty),*) => { $(
        impl Add<$type> for MyFraction {
            type Output = MyFraction;

            fn add(self, rhs: $type) -> Self::Output {
                Self {
                    numerator: self.numerator + rhs as i128 * self.denominator,
                    denominator: self.denominator,
                }
                .simplify()
            }
        }
        impl Add<MyFraction> for $type {
            type Output = MyFraction;

            fn add(self, rhs: MyFraction) -> Self::Output {
                MyFraction {
                    numerator: self as i128 * rhs.denominator + rhs.numerator,
                    denominator: rhs.denominator,
                }
                .simplify()
            }
        }
        )*
    };
}
implAdd!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl Mul<MyFraction> for MyFraction {
    type Output = MyFraction;

    fn mul(self, rhs: MyFraction) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
        .simplify()
    }
}
macro_rules! implMul {
    ($($type:ty),*) => { $(
        impl Mul<$type> for MyFraction {
            type Output = MyFraction;

            fn mul(self, rhs: $type) -> Self::Output {
                Self {
                    numerator: self.numerator * rhs as i128,
                    denominator: self.denominator,
                }
                .simplify()
            }
        }
        impl Mul<MyFraction> for $type {
            type Output = MyFraction;

            fn mul(self, rhs: MyFraction) -> Self::Output {
                MyFraction {
                    numerator: rhs.numerator * self as i128,
                    denominator: rhs.denominator,
                }
                .simplify()
            }
        }
        )*
    };
}
implMul!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl Div<MyFraction> for MyFraction {
    type Output = MyFraction;

    fn div(self, rhs: MyFraction) -> Self::Output {
        assert_ne!(self.denominator * rhs.numerator, 0);
        Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
        .simplify()
    }
}
macro_rules! implDiv {
    ($($type:ty),*) => { $(
        impl Div<MyFraction> for $type {
            type Output = MyFraction;

            fn div(self, rhs: MyFraction) -> Self::Output {
                MyFraction {
                    numerator: (self as i128) * rhs.denominator,
                    denominator: rhs.numerator,
                }
                .simplify()
            }
        }
        impl Div<$type> for MyFraction {
            type Output = MyFraction;

            fn div(self, rhs: $type) -> Self::Output {
                assert_ne!(self.denominator * rhs as i128, 0);
                Self {
                    numerator: self.numerator,
                    denominator: self.denominator * rhs as i128,
                }
                .simplify()
            }
        }
        )*
    };
}
implDiv!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl SubAssign for MyFraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl AddAssign for MyFraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
macro_rules! implSubAddAss {
    ($($type:ty),*) => { $(
        impl SubAssign<$type> for MyFraction {
            fn sub_assign(&mut self, rhs: $type) {
                *self = *self - rhs;
            }
        }
        impl AddAssign<$type> for MyFraction {
            fn add_assign(&mut self, rhs: $type) {
                *self = *self + rhs;
            }
        }
        )*
    };
}
implSubAddAss!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl PartialEq for MyFraction {
    fn eq(&self, other: &Self) -> bool {
        self.denominator == other.denominator && self.numerator == other.numerator
    }
}
impl PartialOrd for MyFraction {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MyFraction {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.denominator == other.denominator {
            return self.numerator.cmp(&other.numerator);
        } else {
            return (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator));
        }
    }
}
macro_rules! implCmp {
    ($($type:ty),*) => { $(
        impl PartialEq<$type> for MyFraction {
            fn eq(&self, other: &$type) -> bool {
                self.denominator == 1 && self.numerator == *other as i128
            }
        }
        impl PartialOrd<$type> for MyFraction {
            fn partial_cmp(&self, other: &$type) -> Option<Ordering> {
                if self.denominator == 1 {
                    return Some(self.numerator.cmp(&(*other as i128)));
                } else {
                    return Some(self.numerator.cmp(&(*other as i128 * self.denominator)));
                }
            }
        }

        impl PartialEq<MyFraction> for $type {
            fn eq(&self, other: &MyFraction) -> bool {
                other.denominator == 1 && other.numerator == *self as i128
            }
        }
        impl PartialOrd<MyFraction> for $type {
            fn partial_cmp(&self, other: &MyFraction) -> Option<Ordering> {
                if other.denominator == 1 {
                    return Some((*self as i128).cmp(&other.numerator));
                } else {
                    return Some(((*self as i128) * other.denominator).cmp(&other.numerator));
                }
            }
        }
        )*
    };
}
implCmp!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

macro_rules! implFrom {
    ($($type:ty),*) => { $(
        impl From<$type> for MyFraction {
            fn from(value: $type) -> Self {
                MyFraction {
                    numerator: value as i128,
                    denominator: 1,
                }
            }
        }
        )*
    };
}
implFrom!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl Display for MyFraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)?;
        } else {
            write!(f, "({}/{})", self.numerator, self.denominator)?;
        }
        Ok(())
    }
}
