use std::fs;
use std::collections::HashMap;
use std::str::Split;
fn main() {

    const FILE_PATH:&'static str = "./src/input.txt";
    let contents:String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
        
    // println!("> File Contents: {}", contents);

    // Regular Results test
    let rounds1: Split<&str> = contents.split("\n");

    let mut total:i32 = 0;

    for round in rounds1 {
        // println!("> {}", round);
        let game: Split<&str> = round.split(" ");
        let vec: Vec<&str> = game.collect();
        let player1 = vec[0];
        let player2: &str = vec[1];
        
        total = total + get_score(player1,player2);
    }
    println!("> Result : {:?}", total);

    // Strategy Test
    let rounds2: Split<&str> = contents.split("\n");
    let mut strategy:HashMap<&str, &str> = HashMap::new();
    strategy.insert("A", "Y");
    strategy.insert("B", "X");
    strategy.insert("C", "Z");


    total = 0;
    for round in rounds2 {
        let game: Split<&str> = round.split(" ");
        let vec: Vec<&str> = game.collect();
        let player1 = vec[0];
        let player2_code: &str = vec[1];
        let result = get_result_ultra_top_secret(player2_code);
        let score = result + generate_hand_score(player1, result);
        total = total + score;
    }

    println!("> Ultra top secret Result : {:?}", total);

}

fn get_result_ultra_top_secret(player2_code: &str) -> i32 {

    // lose
    if player2_code == "X" {
        return 0
    }
    // draw
    if player2_code == "Y" {
        return 3
    }
    return 6
}

fn get_score(player1: &str, player2: &str) ->i32 {
    let mut points:HashMap<&str, i32> = HashMap::new();
    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);

    let choice_score: &i32 = points.get(player2).unwrap();
    let result_score: i32 = result_score(player1, player2_to_player1(player2));
    return choice_score + result_score;
}

fn result_score(player1: &str, player2: &str) -> i32{

    if player1 == player2 {
        return 3;
    }
    if player1 == "A" && player2 == "B" {
        return 6;
    }
    if player1 == "B" && player2 == "C" {
        return 6;
    }
    if player1 == "C" && player2 == "A" {
        return 6;
    }
    return 0;
}

fn player2_to_player1(player2: &str) -> &str {
    if player2 == "X" {
        return "A"
    }
    if player2 == "Y" {
        return "B"
    }
    return "C"
}

fn generate_hand_score(player1: &str, result: i32) -> i32 {
    let mut points:HashMap<&str, i32> = HashMap::new();
    points.insert("A", 1);
    points.insert("B", 2);
    points.insert("C", 3);

    
    if result == 6 {
        if player1 == "A" {
            return 2;
        }
        if player1 == "B" {
            return 3;
        }
        return 1;
    }
    if result == 0 {
        if player1 == "A" {
            return 3;
        }
        if player1 == "B" {
            return 1;
        }
        return 2;
    }

    return *points.get(player1).unwrap();

}