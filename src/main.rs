use std::collections::HashMap;
use rand::Rng;
use std::io::*;

fn get_random_computer_hand()-> String {
    let random_hand = rand::thread_rng().gen_range(1..4);
    let mut ia_hand = String::new();
    match random_hand {
        1=> ia_hand=String::from("rock"),
        2=>ia_hand = String::from("papers"),
        3=>ia_hand = String::from("scissors"),
        _ => ia_hand = String::from("error")
    }
    println!("ia: {}",ia_hand);
    return ia_hand
}


fn main() {

}
