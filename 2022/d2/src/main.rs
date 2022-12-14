use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let input = fs::read_to_string("input.txt")?;

    let mut score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent = chars.next().unwrap();
        let player = chars.nth(1).unwrap();
        let round_score1 = match (opponent, player) {
            ('A', 'Y') => 6,// Rock < Paper
            ('B', 'Z') => 6,// Paper < Scissors
            ('C', 'X') => 6,// Scissors < Rock

            ('A', 'X') => 3,
            ('B', 'Y') => 3,
            ('C', 'Z') => 3,

            ('A', 'Z') => 0,
            ('B', 'X') => 0,
            ('C', 'Y') => 0,
            _ => panic!("Invalid input"),
        };
        let round_score2 = match player {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Invalid input"),
        };
        //println!("{} {} {}", opponent,player,round_score1+round_score2);
        score += round_score1+round_score2;
    }
    println!("{}", score);
    Ok(())
}
