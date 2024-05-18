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


fn run(mut match_points: HashMap<&str, i32>, number_turns: i32){
    let mut computer_hand = get_random_computer_hand();
    computer_hand = computer_hand.trim().parse().unwrap();
    let mut player_hand = ask_player_hand();
    player_hand = player_hand.trim().parse().unwrap();
    if computer_hand == player_hand {
        println!("draw")
    }
    if computer_hand == "rock" && player_hand == "scissors"
        || computer_hand == "scissors" && player_hand == "paper"
        || computer_hand == "paper" && player_hand == "rock"
    {
        println!("computer played {}, it won",computer_hand);
        match_points.insert("computer",match_points["computer"]+1);
    }else {
        println!("you played {}, you won",player_hand);
        match_points.insert("player",match_points["player"]+1);
    }
    if match_points["computer"] == number_turns || match_points["player"] == number_turns {
        println!("final score: \n {:?}",match_points)
    }else {
        run(match_points,number_turns)
    }
}

fn main() {

}
