use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let input = fs::read_to_string("input.txt")?;
    let v: Vec<&str> = input.split_terminator('\n').collect();

    let mut local_total = 0;

    let mut elves_calories = Vec::<i32>::new();

    for i in v {
        if i == "" {
            //println!("{:?}", local_total);

            elves_calories.push(local_total);
            local_total = 0;
        } else {
            local_total += i.parse::<i32>()?;
        }
    }
    //println!("{:?}", local_total);
    elves_calories.push(local_total);

    elves_calories.sort();
    elves_calories.reverse();
    //println!("{:?}", elves_calories);

    let top3 = elves_calories[0..3].iter().sum::<i32>();
    println!("{:?}", top3);

    Ok(())
    
}
