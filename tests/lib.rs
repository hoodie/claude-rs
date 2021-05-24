

use claude::Currency;
use std::cmp::Ordering;

#[test]
fn taxes() {
    let a = Currency{symbol: Some('$'), value: 1000};
    let b = Currency{symbol: Some('$'), value: 1190};

    fn foo(n: f64) -> i64 {
        (n * 1000f64).abs() as i64
    }

    assert_eq!(1 * foo(1.19), 1190);
    assert_eq!(a * 1.19, b);
    assert_eq!(1.19 * a, b);
    assert_eq!(a * 1.1901, b);
    assert_eq!(1.1901 * a, b);
}

#[test]
fn eq_works() {
    let a = Currency{symbol: Some('$'), value: 1210};
    let b = Currency{symbol: Some('$'), value: 1210};
    let c = Currency{symbol: Some('$'), value: 1251};

    assert!(a == b);
    assert!(b == b);
    assert!(b == a);
    assert!(a != c);
}

#[test]
fn ord_works() {
    let a = Currency{symbol: Some('$'), value: 1210};
    let b = Currency{symbol: Some('$'), value: 1211};
    let c = Currency{symbol: Some('$'), value: 1311};
    let d = Currency{symbol: Some('$'), value: 1210};

    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
    assert_eq!(a.partial_cmp(&d), Some(Ordering::Equal));
    assert_eq!(c.partial_cmp(&a), Some(Ordering::Greater));

    assert!(a < b);
    assert!(a < c);
    assert!(a <= a);
    assert!(a <= c);
    assert!(b > a);
    assert!(c > a);
    assert!(a >= a);
    assert!(c >= a);
}

#[test]
fn arithmetic_works() {
    let x = Currency{symbol: Some('$'), value: 1206};
    let y = Currency{symbol: Some('$'), value: 1143};

    assert!(x + y == Currency{ symbol: Some('$'), value: 2349} && y + x == Currency{ symbol: Some('$'), value: 2349});
    assert!(x - y == Currency{ symbol: Some('$'), value: 63});
    assert!(y - x == Currency{ symbol: Some('$'), value: -63});
    assert!(x * 2 == Currency{ symbol: Some('$'), value: 2412} && 2 * x == Currency{ symbol: Some('$'), value: 2412});
    assert!(x / 2 == Currency{ symbol: Some('$'), value: 603});
}

#[test]
#[cfg(feature="parsing")]
fn parse_works() {
    let a1 = Currency{ symbol: Some('$'), value: 1210};
    let b1 = Currency::from_string("$12.10");
    assert!(a1 == b1.unwrap());

    let a2 = Currency{ symbol: Some('$'), value: 1200};
    let b2 = Currency::from_string("$12");
    assert!(a2 == b2.unwrap());

    let a3 = Currency{ symbol: None, value: 1200099};
    let b3 = Currency::from_string("12,000.99");
    assert!(a3 == b3.unwrap());

    let a4 = Currency{ symbol: Some('£'), value: 1200099};
    let b4 = Currency::from_string("£12.000,99");
    assert!(a4 == b4.unwrap());

    // Negatives
    let a5 = Currency{ symbol: Some('$'), value: -1210};
    let b5 = Currency::from_string("-$12.10");
    println!("{:?}, {:?}", a1, b1);
    assert!(a5 == b5.unwrap());

    let a6 = Currency{ symbol: Some('$'), value: -1200};
    let b6 = Currency::from_string("-$12");
    assert!(a6 == b6.unwrap());

    let a7 = Currency{ symbol: None, value: -1200099};
    let b7 = Currency::from_string("-12,000.99");
    assert!(a7 == b7.unwrap());

    let a8 = Currency{ symbol: Some('£'), value: -1200099};
    let b8 = Currency::from_string("-£12.000,99");
    assert!(a8 == b8.unwrap());

    // Zeros
    let a9 = Currency{ symbol: Some('€'), value: 0};
    let b9 = Currency::from_string("€0");
    assert!(a9 == b9.unwrap());

    let a10 = Currency{ symbol: None, value: 0};
    let b10 = Currency::from_string("000");
    assert!(a10 == b10.unwrap());

    let a11 = Currency{ symbol: Some('€'), value: 50};
    let b11 = Currency::from_string("€0,50");
    assert!(a11 == b11.unwrap());

    let a12 = Currency{ symbol: Some('€'), value: -50};
    let b12 = Currency::from_string("-€0.50");
    assert!(a12 == b12.unwrap());
}

#[test]
fn display_works() {
    assert_eq!(Currency{ symbol: None, value: 1210}.prefix().to_string(), "12.10");
    assert_eq!(Currency{ symbol: Some('$'), value: 1210}.prefix().to_string(), "$12.10");
    assert_eq!(Currency{ symbol: Some('$'), value: 100010}.prefix().to_string(), "$1000.10");

    // Not implemented
    //assert!(format!("{:e}", Currency{ symbol: Some('£'), value: 100000}) == "£1000,00");
}


#[test]
fn default() {
    assert_eq!(Currency::default(), Currency{ symbol: None, value: 0});
}


#[test]
fn add_to_default() {
    assert_eq!(Currency::default() + Currency{ symbol: Some('€'), value: 1}, Currency{ symbol: Some('€'), value: 1});
}
