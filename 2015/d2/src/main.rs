use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let input = fs::read_to_string("input.txt")?;
    let mut floor = 0;
    let mut position = 0;
    for c in input.chars() {
        position += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            println!("Santa entered the basement at position {}", position);
            break;
        }
    }
    Ok(())
}
