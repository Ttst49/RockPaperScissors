use std::collections::HashMap;
use rand::Rng;
fn get_random_hand()-> &'static str {
    let random_hand = rand::thread_rng().gen_range(1..4);
    let ia_hand : &str;
    match random_hand {
        1=> ia_hand="rock",
        2=>ia_hand = "papers",
        3=>ia_hand = "scissors",
        _ => ia_hand = "null"
    }
    ia_hand
}


fn run(){
    let mut match_points =  HashMap::from([
        ("computer",0),
        ("player",0)
    ]);



}

fn main() {
    let value = get_random_hand();
    println!("{:?}", value);
}
