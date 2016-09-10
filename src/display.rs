//! `Display` Implementation.

use std::fmt;
use ::Currency;

/// Allows Currencies to be displayed as Strings. The format includes no comma delimiting with a
/// two digit precision decimal.
///
/// # Examples
/// ```
/// use currency::Currency;
///
/// assert!(Currency(Some('$'), 1210).to_string() == "$12.10");
/// assert!(Currency(None, 1210).to_string() == "12.10");
///
/// println!("{}", Currency(Some('$'), 100099));
/// ```
/// The last line prints the following:
/// ```text
/// "$1000.99"
/// ```
impl fmt::Display for Currency {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let decimal = format!("{:.2}", (self.1 as f32 / 100.0));
        match self.0 {
            Some(c) => write!(f, "{}{}", c, decimal),
            None => write!(f, "{}", decimal),
        }
    }
}

/// Identical to the implementation of Display, but replaces the "." with a ",". Access this
/// formating by using "{:e}".
///
/// # Examples
/// ```
/// use currency::Currency;
///
/// println!("{:e}", Currency(Some('£'), 100099));
/// ```
/// The last line prints the following:
/// ```text
/// "£1000,99"
/// ```
impl fmt::LowerExp for Currency {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{}", self).replace(".", ","))
    }
}
