extern crate claude;

use claude::*;

fn main(){
    let my_money = Currency{symbol: Some('€'), value: 4711};

    println!("I have {:?}!", my_money);
    //println!("I have {}!", my_money); // doesn't compile :D
    println!("I have {}!", my_money.postfix()); // -> "I have 47,11€!"
    println!("I have {}!", my_money.prefix());  // -> "I have €47.11!"
}
