use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("Advent of Code 2022: Day 2 - Rock Paper Scissors");
    part_one();
    part_two();
}

//Function returns an iterator to the reader of the lines of the file
fn read_lines<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new(file).lines())
}

fn part_one() {
    //read through the input file and break each line into a win or a loss with points
    let mut jenken_results : Vec<u32> = vec![];
        
    if let Ok(lines) = read_lines("./data/input.txt" ) {
        //consumes the iterator and returns an optional string -- I need to read up on what this means
        for line in lines {
            if let Ok(ip) = line {
                //We have a line from the file if it is not blank append to a vector
                //split the line on the space character
                let round = ip.split(" ");
                let mut round_values: Vec<&str> = round.collect();
                
                match round_values[0] {
                    "A" => round_values[0] = "X",
                    "B" => round_values[0] = "Y",
                    "C" => round_values[0] = "Z",
                    &_ => println!("Invalid Result"),
                }

                let mut score : u32 = 0;

                //it's a draw
                if round_values[1] == round_values[0]  {
                    score += 3;
                   
                }

                if round_values[1] == "X" && round_values[0] == "Z"  ||
                   round_values[1] == "Y" && round_values[0] == "X"  ||
                   round_values[1] == "Z" && round_values[0] == "Y" {
                    score += 6;
                }

                match round_values[1] {
                    "X" => score += 1,
                    "Y" => score += 2,
                    "Z" => score += 3,
                    &_ => println!("Invalid Result"),
                }
                jenken_results.push(score);
                               
            }
        }
   }
   //Why not print out how many elves are on the trip?
    println!("Number of Rounds of Jenken: {}", jenken_results.len());
    
    //use the iter.max function to find the elf with the most calories
    let total_score : u32 = jenken_results.iter().sum();
    //Print the answer for part 2
    println!( "Top total score is: {}", total_score );
}

fn part_two() {
    //read through the input file and break each line into a win or a loss with points
    let mut jenken_results : Vec<u32> = vec![];
    let response_values = vec![ "A", "B", "C" ];

    if let Ok(lines) = read_lines("./data/input.txt" ) {
        //consumes the iterator and returns an optional string -- I need to read up on what this means
        for line in lines {
            if let Ok(ip) = line {
                //We have a line from the file if it is not blank append to a vector
                //split the line on the space character
                let round = ip.split(" ");
                let mut round_values: Vec<&str> = round.collect();
                
                match round_values[1] {
                    "X" => round_values[1] = response_values[(response_values.iter().position(|&r| r == round_values[0]).unwrap() + 2) % 3],
                    "Y" => round_values[1] = round_values[0],
                    "Z" => round_values[1] = response_values[(response_values.iter().position(|&r| r == round_values[0]).unwrap() + 1) % 3],
                    &_ => println!("Invalid Result"),
                }

                println!( "Fixed Round {} {}", round_values[0], round_values[1]);
                let mut score : u32 = 0;

                //it's a draw
                if round_values[0] == round_values[1]  {
                    score += 3;
                }

                if round_values[1] == "A" && round_values[0] == "C"  ||
                   round_values[1] == "B" && round_values[0] == "A"  ||
                   round_values[1] == "C" && round_values[0] == "B" {
                    score += 6;
                }

                match round_values[1] {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    &_ => println!("Invalid Result"),
                }
                jenken_results.push(score);
                               
            }
        }
   }
   //Why not print out how many elves are on the trip?
    println!("Number of Rounds of Jenken: {}", jenken_results.len());
    
    //use the iter.max function to find the elf with the most calories
    let total_score : u32 = jenken_results.iter().sum();
    //Print the answer for part 2
    println!( "Top total score is: {}", total_score );
}