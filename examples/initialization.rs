extern crate claude;

use claude::*;

fn display(my_money: Currency){
    println!("I have {:?}!", my_money);
    //println!("I have {}!", my_money); // doesn't compile :D
    println!("I have {}!", my_money.postfix()); // -> "I have 47,11€!"
    println!("I have {}!", my_money.prefix());  // -> "I have €47.11!"
    println!()
}

fn main() {
    display(Currency::from(50));
    display(Currency::from((50, '€')));
    display(Currency::from(('$', 50)));
}
