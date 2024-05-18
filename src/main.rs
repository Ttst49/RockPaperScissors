use std::collections::HashMap;
use rand::Rng;
use std::io::*;

fn get_random_computer_hand()-> String {
    let random_hand = rand::thread_rng().gen_range(1..4);

    return match random_hand {
        1 => String::from("rock"),
        2 => String::from("papers"),
        3 => String::from("scissors"),
        _ => String::from("error")
    }
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
        return player_hand
    }else {
        println!(
            "You have to choose a correct answer. \n
        No, well is not a valid answer cheater"
        );
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
    let match_points =  HashMap::from([
        ("computer",0),
        ("player",0)
    ]);
    run(match_points,3)
}
