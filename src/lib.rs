//! This provides a simple type to calculate with monetary values.

#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
    )]

#[cfg(feature="parsing")] extern crate regex;
#[cfg(feature="serialization")] extern crate serde;
#[cfg(feature="serialization")] extern crate serde_json;
#[cfg(feature="serialization")] #[macro_use] extern crate serde_derive;

use std::default::Default;

pub mod display;
pub mod math;

/// Represents currency through an optional symbol and amount of coin.
///
/// Each 100 coins results in a banknote. (100 is formatted as 1.00)
/// The currency will be formatted as such: `Currency(Some('$'), 432)` ==> "$4.32"
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
#[cfg_attr(feature = "serialization", derive(Serialize))]
pub struct Currency {
    /// Currency symbol
    ///
    /// pick any of `€`, `£`, `$`, `¥` etc...
    pub symbol: Option<char>,
    /// value in the smallest possible unit
    pub value: i64
}

impl Currency {
    /// Creates a blank Currency as Currency(None, 0)
    ///
    /// # Examples
    /// ```
    /// # use claude::Currency;
    /// let mut c = Currency::new();
    /// ```
    #[inline]
    pub fn new() -> Currency {
        Currency{symbol: None, value: 0}
    }

    /// Initialize from `i64`
    pub fn from_value(value: i64) -> Currency {
        Currency{symbol: None, value: value}
    }

    /// Uses a Regular Expression to parse a string literal (&str) and attempts to turn it into a
    /// currency. Returns `Some(Currency)` on a successful conversion, otherwise `None`.
    ///
    /// If the currency is intended to be a negative amount, ensure the '-' is the first character
    /// in the string.
    /// The Regex recognizes European notation (€1,00)
    ///
    /// # Examples
    /// ```
    /// use claude::Currency;
    ///
    /// assert!(Currency::from_string("$4.32")  == Some(Currency{symbol: Some('$'), value:   432}));
    /// assert!(Currency::from_string("-$4.32") == Some(Currency{symbol: Some('$'), value:  -432}));
    /// assert!(Currency::from_string("424.44") == Some(Currency{symbol: None,      value: 42444}));
    /// assert!(Currency::from_string("£12,00") == Some(Currency{symbol: Some('£'), value:  1200}));
    /// assert!(Currency::from_string("¥12")    == Some(Currency{symbol: Some('¥'), value:  1200}));
    /// ```
    #[cfg(feature="parsing")]
    pub fn from_string(s: &str) -> Option<Currency> {
        use regex::Regex;

        // Shadow s with a trimmed version
        let s = s.trim();
        let re =
            Regex::new(r"(?:\b|(-)?)(\p{Sc})?((?:(?:\d{1,3}[\.,])+\d{3})|\d+)(?:[\.,](\d{2}))?\b")
            .unwrap();

        if !re.is_match(s) {
            return None;
        }

        // Used to negate the final result if the regex matches a negative
        let mut multiplier = 1;
        let mut sign: Option<char> = None;
        let mut coin_str: String = "".to_string();

        // If anyone's looking at this and knows how to do this without a loop, fork this.
        for cap in re.captures_iter(s) {
            if cap.at(0).unwrap_or("") != s {
                return None;
            }

            if cap.at(1).is_some() {
                multiplier = -1;
            }

            if cap.at(2).is_some() {
                if multiplier < 0 {
                    sign = Some(s.chars().skip(1).next().unwrap());
                } else {
                    sign = Some(s.chars().next().unwrap());
                }
            }
            coin_str = cap.at(3).unwrap().replace(".", "").replace(",", "")
                     + cap.at(4).unwrap_or("00");

            break;
        }

        if let Ok(coin) = coin_str.parse::<i64>(){
            return Some(Currency{symbol: sign, value: multiplier * coin});
        }
        None
    }

    /// Returns an object that implements `Display` for different methods of printing currency.
    pub fn postfix(&self) -> Postfix {
        Postfix{money: self}
    }

    /// Returns an object that implements `Display` for different methods of printing currency.
    pub fn prefix(&self) -> Prefix {
        Prefix{money: self}
    }

    /// Returns the value as float
    ///
    /// # Warning, do not use this for calculation, this is for displaying only!
    pub fn as_float(&self) -> f64{
        self.value as f64 / 100.0
    }

    /// Returns the inner value
    pub fn value(&self) -> i64{
        self.value
    }

    /// Returns the inner symbol
    pub fn symbol(&self) -> Option<char>{
        self.symbol
    }
}

use std::ops::Deref;
/// Required for `DerefMut`
impl Deref for Currency{
    type Target = i64;
    fn deref(&self) -> &i64{
        &self.value
    }
}

impl From<i64> for Currency {
    /// converts from a  `i64`
    fn from(value:i64) -> Currency {
        Currency::from_value(value)
    }
}

impl From<(char,i64)> for Currency {
    /// converts from a tuple of `char` and `i64`
    fn from(tpl:(char, i64)) -> Currency {
        let (symbol, cents) = tpl;
        Currency{symbol: Some(symbol), value: cents}
    }
}

impl From<(i64,char)> for Currency {
    /// converts from a tuple of `i64` and `char`
    fn from(tpl:(i64, char)) -> Currency {
        let (cents, symbol) = tpl;
        Currency{symbol: Some(symbol), value: cents}
    }
}

impl Default for Currency{
    fn default() -> Self{
        Currency{symbol: None, value: 0}
    }
}

/// Implements `Display` with the currency symbol at the end.
pub struct Postfix<'a>{
    money: &'a Currency
}

/// Implements `Display` with the currency symbol at the front.
pub struct Prefix<'a>{
    money: &'a Currency
}

