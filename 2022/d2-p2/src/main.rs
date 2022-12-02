use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let input = fs::read_to_string("input.txt")?;

    let mut score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent = chars.next().unwrap();
        let player = chars.nth(1).unwrap();
        let round_score2 = match player {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Invalid input"),
        };
        let round_score1 = match (opponent, round_score2) {
            ('A', 0) => 3,// Rock > Scissors
            ('A', 3) => 1,// Rock = Rock
            ('A', 6) => 2,// Rock < Paper

            ('B', 0) => 1,// Paper > Rock
            ('B', 3) => 2,// Paper = Paper
            ('B', 6) => 3,// Paper < Scissors

            ('C', 0) => 2,// Scissors > Paper
            ('C', 3) => 3,// Scissors = Scissors
            ('C', 6) => 1,// Scissors < Rock
            _ => panic!("Invalid input"),
        };

        //println!("{} {} {}", opponent,player,round_score1+round_score2);
        score += round_score1+round_score2;
    }
    println!("{}", score);
    Ok(())
}
