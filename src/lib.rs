use std::cmp;
use std::cmp::PartialEq;
use std::convert::From;
use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
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
        if self.denominator.is_negative() {
            self.numerator *= -1;
            self.denominator *= -1;
        }
        self.numerator /= max_factor;
        self.denominator /= max_factor;
        assert_ne!(self.denominator, 0);
        self
    }
}
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
impl Sub<i128> for MyFraction {
    type Output = MyFraction;

    fn sub(self, rhs: i128) -> Self::Output {
        Self {
            numerator: self.numerator - rhs * self.denominator,
            denominator: self.denominator,
        }
        .simplify()
    }
}
impl Sub<MyFraction> for i128 {
    type Output = MyFraction;

    fn sub(self, rhs: MyFraction) -> Self::Output {
        MyFraction {
            numerator: self * rhs.denominator - rhs.numerator,
            denominator: rhs.denominator,
        }
        .simplify()
    }
}
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
impl Add<i128> for MyFraction {
    type Output = MyFraction;

    fn add(self, rhs: i128) -> Self::Output {
        Self {
            numerator: self.numerator + rhs * self.denominator,
            denominator: self.denominator,
        }
        .simplify()
    }
}
impl Add<MyFraction> for i128 {
    type Output = MyFraction;

    fn add(self, rhs: MyFraction) -> Self::Output {
        MyFraction {
            numerator: self * rhs.denominator + rhs.numerator,
            denominator: rhs.denominator,
        }
        .simplify()
    }
}
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
impl Mul<i128> for MyFraction {
    type Output = MyFraction;

    fn mul(self, rhs: i128) -> Self::Output {
        Self {
            numerator: self.numerator * rhs,
            denominator: self.denominator,
        }
        .simplify()
    }
}
impl Mul<MyFraction> for i128 {
    type Output = MyFraction;

    fn mul(self, rhs: MyFraction) -> Self::Output {
        MyFraction {
            numerator: rhs.numerator * self,
            denominator: rhs.denominator,
        }
        .simplify()
    }
}
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
impl Div<i128> for MyFraction {
    type Output = MyFraction;

    fn div(self, rhs: i128) -> Self::Output {
        assert_ne!(self.denominator * rhs, 0);
        Self {
            numerator: self.numerator,
            denominator: self.denominator * rhs,
        }
        .simplify()
    }
}
impl Div<MyFraction> for i128 {
    type Output = MyFraction;

    fn div(self, rhs: MyFraction) -> Self::Output {
        MyFraction {
            numerator: self * rhs.denominator,
            denominator: rhs.numerator,
        }
        .simplify()
    }
}
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
impl PartialEq for MyFraction {
    fn eq(&self, other: &Self) -> bool {
        self.denominator == other.denominator && self.numerator == other.numerator
    }
}
impl From<i128> for MyFraction {
    fn from(value: i128) -> Self {
        MyFraction {
            numerator: value,
            denominator: 1,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        println!("MY PRINTS:");
        const N: i128 = 4;

        let fr = MyFraction::new(2, 4);
        let fr_2 = MyFraction::new(3, 8);
        let fr_3 = MyFraction::new(3, 2);
        let fr_4 = MyFraction::new(1, 4);

        println!("{fr_3} - {fr_4} = {}", fr_3 - fr_4);
        println!("{fr_3} - {N}(i128) = {}", fr_3 - N);
        println!("{N}(i128) - {fr_3} = {}", N - fr_3);
        println!();
        println!("{fr} + {fr_2} = {}", fr + fr_2);
        println!("{fr_3} + {fr_4} = {}", fr_3 + fr_4);
        println!("{fr_3} + {N}(i128) = {}", fr_3 + N);
        println!("{N}(i128) + {fr_3} = {}", N + fr_3);
        println!();
        println!("{fr_3} / {fr_4} = {}", fr_3 / fr_4);
        println!("{fr_3} / {N}(i128) = {}", fr_3 / N);
        println!("{N}(i128) / {fr_3} = {}", N / fr_3);
        println!();
        println!("{fr_3} * {fr_4} = {}", fr_3 * fr_4);
        println!("{fr_3} * {N}(i128) = {}", fr_3 * N);
        println!("{N}(i128) * {fr_3} = {}", N * fr_3);
        println!();

        let fr_5 = MyFraction::new(-2, 3);
        println!("{fr_3} / {fr_5} = {}", fr_3 / fr_5);

        panic!("OK PANIC");
    }
}
