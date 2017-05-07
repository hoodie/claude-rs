#![cfg(feature="serialization")]
extern crate claude;
extern crate serde_json;

use claude::*;

fn main() {
    let my_money = Currency{ symbol: Some('€'), value: 4711};

    println!("{}", serde_json::to_string(&my_money).unwrap());
}
