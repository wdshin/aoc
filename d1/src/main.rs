use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let input = fs::read_to_string("input.txt")?;
    let v: Vec<&str> = input.split_terminator('\n').collect();
    //println!("{:?}", v);
    let mut max_total = 0;
    let mut local_total = 0;

    for i in v {
        if i == "" {
            if local_total > max_total {
                //println!("new max {} > {}", local_total,max_total);
                max_total = local_total;
            } else {
                //println!("old max {} < {}", local_total,max_total);
            } 
            local_total = 0;
        } else {
            local_total += i.parse::<i32>()?;
        }
    }

    if local_total > max_total {

        //println!("final new max {} > {}", local_total,max_total);

        max_total = local_total;
    }

    println!("{:?}", max_total);

    Ok(())
    
}
