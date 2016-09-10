//! Implementations of standard operators:  `Add`, `Sub`, `Mul`, `Div`

use std::ops::{Add, Sub, Mul, Div};
use ::Currency;

/// Overloads the '+' operator for Currency objects.
///
/// # Panics
/// Panics if the two addends are different types of currency, as denoted by the Currency's symbol.
impl Add for Currency {
    type Output = Currency;

    #[inline]
    fn add(self, rhs: Currency) -> Currency {
        if self.0 == rhs.0 || self.0.is_none() {
            Currency(rhs.0, self.1 + rhs.1)
        } else {
            panic!("Cannot add two different types of currency!\n{:?} vs {:?}",
                   self.0,
                   rhs.0);
        }
    }
}

/// Overloads the '-' operator for Currency objects.
///
/// # Panics
/// Panics if the minuend and subtrahend are two different types of currency, as denoted by the
/// Currency's symbol.
impl Sub for Currency {
    type Output = Currency;

    #[inline]
    fn sub(self, rhs: Currency) -> Currency {
        if self.0 == rhs.0 {
            Currency(self.0, self.1 - rhs.1)
        } else {
            panic!("Cannot subtract two different types of currency!");
        }
    }
}

/// Overloads the '*' operator for Currency objects.
///
/// Allows a Currency to be multiplied by an i64.
impl Mul<i64> for Currency {
    type Output = Currency;

    #[inline]
    fn mul(self, rhs: i64) -> Currency {
        Currency(self.0, self.1 * rhs)
    }
}

/// Overloads the '*' operator for i64.
///
/// Allows an i64 to be multiplied by a Currency.
/// Completes the commutative property for i64 multiplied by Currency.
impl Mul<Currency> for i64 {
    type Output = Currency;

    #[inline]
    fn mul(self, rhs: Currency) -> Currency {
        Currency(rhs.0, rhs.1 * self)
    }
}

/// Multiplies with float, probably not a good idea, help appreciated.
impl Mul<f64> for Currency {
    type Output = Currency;

    #[inline]
    fn mul(self, rhs: f64) -> Currency {
        Currency(self.0, (self.1 as f64 * rhs).round() as i64)
    }
}

impl Mul<Currency> for f64 {
    type Output = Currency;

    #[inline]
    fn mul(self, rhs: Currency) -> Currency {
        rhs * self
    }
}

/// Overloads the '/' operator for Currency objects.
///
/// Allows a Currency to be divided by an i64.
impl Div<i64> for Currency {
    type Output = Currency;

    #[inline]
    fn div(self, rhs: i64) -> Currency {
        Currency(self.0, self.1 / rhs)
    }
}
