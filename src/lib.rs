//! This provides a simple type that encodes a currency.

#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
    )]


extern crate regex;

pub mod display;
pub mod math;
pub mod sum;


/// Represents currency through an optional symbol and amount of coin.
///
/// Each 100 coins results in a banknote. (100 is formatted as 1.00)
/// The currency will be formatted as such: `Currency(Some('$'), 432)` ==> "$4.32"
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct Currency(pub Option<char>, pub i64);

impl Currency {
    /// Creates a blank Currency as Currency(None, 0)
    ///
    /// # Examples
    /// ```
    /// use currency::Currency;
    ///
    /// let mut c = Currency::new();
    /// ```
    #[inline]
    pub fn new() -> Currency {
        Currency(None, 0)
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
    /// use currency::Currency;
    ///
    /// assert!(Currency::from_string("$4.32")  == Some(Currency(Some('$'), 432)));
    /// assert!(Currency::from_string("-$4.32") == Some(Currency(Some('$'), -432)));
    /// assert!(Currency::from_string("424.44") == Some(Currency(None, 42444)));
    /// assert!(Currency::from_string("£12,00") == Some(Currency(Some('£'), 1200)));
    /// assert!(Currency::from_string("¥12")    == Some(Currency(Some('¥'), 1200)));
    /// ```
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
            return Some(Currency(sign, multiplier * coin));
        }
        None
    }
}

use std::default::Default;

impl Default for Currency{
    fn default() -> Self{
        Currency(None, 0)
    }
}
