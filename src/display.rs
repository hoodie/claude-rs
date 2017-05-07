//! `Display` Implementation.

use std::fmt;
use std::ops::Deref;

use super::{Currency, Postfix, Prefix};

macro_rules! impl_deref_to_currency{
    ($s:ty) => {
        impl<'a> Deref for $s {
            type Target = Currency;

            fn deref(&self) -> &Currency {
                &self.money
            }
        }
    }
}

impl_deref_to_currency!(Postfix<'a>);
impl_deref_to_currency!(Prefix<'a>);

/// Allows Currencies to be displayed as Strings.
/// The format includes no comma delimiting with a two digit precision decimal.
///
/// # Examples
/// ```
/// use claude::Currency;
///
/// assert!(Currency{ symbol: None, value: 1210}.postfix().to_string() == "12,10");
///
/// println!("{}", Currency{ symbol: Some('€'), value: 100099}.postfix());
/// ```
/// The last line prints the following:
/// ```text
/// "1000,99€"
/// ```
impl<'a> fmt::Display for Postfix<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let decimal = format!("{:.2}", (self.value as f32 / 100.0)).replace(".", ",");
        match self.symbol {
            Some(symbol) => write!(f, "{d}{s}", s = symbol, d = decimal),
            None => write!(f, "{}", decimal),
        }
    }
}

/// Allows Currencies to be displayed as Strings. The format includes no comma delimiting with a
/// two digit precision decimal.
///
/// # Examples
/// ```
/// use claude::Currency;
///
/// assert!(Currency{ symbol: Some('$'), value: 1210}.prefix().to_string() == "$12.10");
/// assert!(Currency{ symbol: None, value: 1210}.prefix().to_string() == "12.10");
///
/// println!("{}", Currency{ symbol: Some('$'), value: 100099}.prefix());
/// ```
/// The last line prints the following:
/// ```text
/// "$1000.99"
/// ```
impl<'a> fmt::Display for Prefix<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let decimal = format!("{:.2}", (self.value as f32 / 100.0));
        match self.symbol {
            Some(symbol) => write!(f, "{s}{d}", s = symbol, d = decimal),
            None => write!(f, "{}", decimal),
        }
    }
}
