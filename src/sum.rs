//! Implementation of `std::iter::Sum`

use ::Currency;

use std::iter::Sum;
impl Sum for Currency {
    fn sum<I>(iter: I) -> Self
        where I: Iterator<Item = Currency>
    {
        iter.fold(Currency::default(), |acc, x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use ::Currency;
    #[test]
    fn sum_up() {
        let list = vec![
            Currency(Some('£'),1000),
            Currency(Some('£'),0100),
            Currency(Some('£'),0010),
            Currency(Some('£'),0001),
        ];

        let fold = list.iter().fold(Currency::default(), |acc, x| acc + *x);
        assert_eq!(fold, Currency(Some('£'),1111));

        let sum:Currency = list.into_iter().sum();
        assert_eq!(sum, Currency(Some('£'),1111));
    }
}
