use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("Advent of Code 2022: Day 1 - Most Calories");
    //Ok so we need to store each elf's total calories in a vector
    let mut elves_calories : Vec<u32> = vec![];
    //use a mutable u32 variable to keep track of the total calories each elf is carrying
    let mut calorie_total = 0;
    if let Ok(lines) = read_lines("./data/input.txt" ) {
        //consumes the iterator and returns an optional string -- I need to read up on what this means
        for line in lines {
            if let Ok(ip) = line {
                //We have a line from the file if it is not blank append to a vector
                //when the line is blank we are onto the next elf so push calorie total into the vector and reset the accumulator value
                if ip.len() == 0 {
                    elves_calories.push(calorie_total);
                    calorie_total = 0;
                }
                else {
                    //We read the file in as a string so convert to a number
                    //rust requires some error handling around this hence the two paths for this is ok/not ok
                    match ip.parse::<u32>() {
                        Ok(n) => calorie_total += n,
                        Err(e) => println!("Help this is wrong {e}"),
                    };
                }
            }
        }
   }
   //Why not print out how many elves are on the trip?
    println!("Number of Elves on trip: {}", elves_calories.len());
    
    //use the iter.max function to find the elf with the most calories
    let max_value = elves_calories.iter().max();
    match max_value {
        None => println!("No max value was located!"),
        Some(i) => println!( "Most Calories: {}", i),
    }

    
    //Part 2 of the question requires total calories for the 3 elves with the most calories
    //Sort the vector to get the elves with the most calories in first place 
    elves_calories.sort_by( | a, b| b.cmp(&a) );
    //Nice that Iterators have a sum function on them and I figured out the syntax for only do this on the first 3 elements!
    let top_three_total_calories : u32 = elves_calories[0..3].iter().sum();
    //Print the answer for part 2
    println!( "Top 3 total calories: {}", top_three_total_calories );

}

//Function returns an iterator to the reader of the lines of the file
fn read_lines<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new(file).lines())
}

