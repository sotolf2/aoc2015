use std::fs::File;
use std::io::{BufRead, self, Error};
use std::env;
use std::cmp;
use regex::Regex;

#[macro_use] extern crate lazy_static;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse(in_line: Result<String, Error>) -> Result<Ingredient, Error> {
    let line = in_line?;
    lazy_static! {
        static ref NAME: Regex = Regex::new(r"^(\w+)").unwrap();
        static ref VALUES: Regex = Regex::new(r"(-?\d)").unwrap();
    }
    let name = NAME.captures(&line).unwrap()[0].to_string();
    let values: Vec<i32> = VALUES.captures_iter(&line)
        .map(|x| x[1].parse::<i32>().unwrap())
        .collect();

    Ok(Ingredient {name, 
        capacity: values[0],
        durability: values[1],
        flavor: values[2],
        texture: values[3],
        calories: values[4]})
}

fn get_input(filename: &String) -> io::Result<Vec<Ingredient>> {
    let file = File::open(filename)?;
    io::BufReader::new(file)
        .lines()
        .map(|x| parse(x))
        .collect()
}

fn part1(ingredients: &[Ingredient]) {
    let mut max = 0;
    for x in 0..=100 {
        for y in 0..=(100 - x) {
            for z in 0..=(100 - x - y) {
                let a = 100 - x - y - z;
                let capacity = x * ingredients[0].capacity + y * ingredients[1].capacity + z * ingredients[2].capacity + a * ingredients[3].capacity;
                let durability = x * ingredients[0].durability + y * ingredients[1].durability + z * ingredients[2].durability + a * ingredients[3].durability;
                let flavor = x * ingredients[0].flavor + y * ingredients[1].flavor + z * ingredients[2].flavor + a * ingredients[3].flavor;
                let texture = x * ingredients[0].texture + y * ingredients[1].texture + z * ingredients[2].texture + a * ingredients[3].texture;
                max = cmp::max(max, cmp::max(0, capacity) * cmp::max(0, durability) * cmp::max(0, flavor) * cmp::max(0, texture));
            }
        }
    }

    println!("Part1: {}", max);

}

fn part2(ingredients: &[Ingredient]) {
    let mut max = 0;
    for x in 0..=100 {
        for y in 0..=(100 - x) {
            for z in 0..=(100 - x - y) {
                let a = 100 - x - y - z;
                let capacity = x * ingredients[0].capacity + y * ingredients[1].capacity + z * ingredients[2].capacity + a * ingredients[3].capacity;
                let durability = x * ingredients[0].durability + y * ingredients[1].durability + z * ingredients[2].durability + a * ingredients[3].durability;
                let flavor = x * ingredients[0].flavor + y * ingredients[1].flavor + z * ingredients[2].flavor + a * ingredients[3].flavor;
                let texture = x * ingredients[0].texture + y * ingredients[1].texture + z * ingredients[2].texture + a * ingredients[3].texture;
                let calories = x * ingredients[0].calories + y * ingredients[1].calories + z * ingredients[2].calories + a * ingredients[3].calories;
                if calories == 500 {
                    max = cmp::max(max, cmp::max(0, capacity) * cmp::max(0, durability) * cmp::max(0, flavor) * cmp::max(0, texture));
                }
            }
        }
    } 
    println!("Part2: {}", max);

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed = get_input(&args[1]);
    
    if let Ok(ingredients) = parsed {
        part1(&ingredients);
        part2(&ingredients);
    }
}
