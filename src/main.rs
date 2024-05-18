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

fn ask_player_hand() -> String {
    println!(
        "what do you wanna play now? \n
        Rock, Paper or scissors
    "
    );
    let mut player_hand = String::new();
    stdin().read_line(&mut player_hand).expect("error: unable to read user input");
    if
    player_hand
        .trim()
        .to_lowercase() == "paper"
        || player_hand
        .trim()
        .to_lowercase() == "rock"
        || player_hand
        .trim()
        .to_lowercase() == "scissors"
    {
        println!("you choose {}",player_hand);
        return player_hand
    }else {
        ask_player_hand()
    }
}


fn main() {

}
