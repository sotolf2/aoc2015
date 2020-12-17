use std::fs::File;
use std::io::{BufRead, self, Error};
use std::env;
use std::collections::HashMap;
use regex::Regex;

#[macro_use] extern crate lazy_static;

fn parse(in_line: Result<String, Error>) -> Result<(i32, HashMap<String,i32>), Error> {
    let line = in_line?;
    lazy_static! {
        static ref SUE: Regex = Regex::new(r"^\w+\s(\d+):").unwrap();
        static ref VALUES: Regex = Regex::new(r"\s(\w+):\s(\d+)").unwrap();
    }
    let sue_nr = SUE.captures(&line).unwrap()[1].parse::<i32>().unwrap();
    let values: HashMap<String,i32> = VALUES.captures_iter(&line)
        .map(|x| (x[1].to_string() ,x[2].parse::<i32>().unwrap()))
        .collect();

    Ok((sue_nr, values))
}

fn get_input(filename: &String) -> io::Result<Vec<(i32, HashMap<String,i32>)>> {
    let file = File::open(filename)?;
    io::BufReader::new(file)
        .lines()
        .map(|x| parse(x))
        .collect()
}

fn is_right_sue(sue: &(i32, HashMap<String,i32>)) -> bool {
    let checks = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];
    let (_, values) = sue;

    for (string, num) in checks.iter() {
        if values.contains_key(*string) {
            if values[*string] != *num {
                return false 
            }
        }
        
    }
    true
}

fn part1(sues: &[(i32, HashMap<String,i32>)]) {
    let sue: Vec<i32> = sues.iter()
                            .filter(|sue| is_right_sue(*sue))
                            .map(|(nr,_)| *nr)
                            .collect();
    println!("Part1: {:?}", sue[0]);

}

fn is_right_sue2(sue: &(i32, HashMap<String,i32>)) -> bool {
    let checks = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];
    let (_, values) = sue;
    //println!("-- sue {}: {:?}", *sue_nr, values );
    for (string, num) in checks.iter() {
        if values.contains_key(*string) {
            let sue_value = values[*string];
            //println!("-- #checking: {}, sue: {}, test: {}", *string, sue_value, num);
            let valid = match (*string, sue_value, num) {
                ("cats", sue, check) => sue > *check,
                ("trees", sue, check) => sue > *check,
                ("pomeranians", sue, check) => sue < *check,
                ("goldfish", sue, check) => sue < *check,
                (_,sue, check) => sue == *check,
            };
            //println!("--- {}", valid);
            if !valid {
                return false 
            }
        }
    }
    true
}

fn part2(sues: &[(i32, HashMap<String,i32>)]) {
    let sue: Vec<i32> = sues.iter()
                            .filter(|sue| is_right_sue2(*sue))
                            .map(|(nr,_)| *nr)
                            .collect();
    println!("Part2: {:?}", sue[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed = get_input(&args[1]);
    
    if let Ok(sues) = parsed {
        //println!("{:?}", sues);
        part1(&sues);
        part2(&sues);
    }
}
