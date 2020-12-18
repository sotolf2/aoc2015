use std::fs::File;
use std::io::{BufRead, self, Error};
use std::env;
use itertools::Itertools;

fn parse(in_line: Result<String, Error>) -> Result<i32, Error> {
    let line = in_line?;
    let num = line.parse::<i32>().expect("Line is not a number");
    Ok(num)
}

fn get_input(filename: &String) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    io::BufReader::new(file)
        .lines()
        .map(|x| parse(x))
        .collect()
}

fn part1(containers: &[i32]) {
    let mut total = 0;
    for i in 1..containers.len() {
        let subtotal = containers.iter()
                        .combinations(i)
                        .map(|x| x.into_iter().sum::<i32>())
                        .filter(|x| x == &150)
                        .count();
        total += subtotal;
    }
    println!("Part1: {}", total)

}

fn part2(containers: &[i32]) {
        for i in 1..containers.len() {
            let subtotal = containers.iter()
                            .combinations(i)
                            .map(|x| x.into_iter().sum::<i32>())
                            .filter(|x| x == &150)
                            .count();
            if subtotal > 0 {
                println!("Part2: {}", subtotal);
                return;
            }
        } 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed = get_input(&args[1]);
    
    if let Ok(numbers) = parsed {
        println!("{:?}", numbers);
        part1(&numbers);
        part2(&numbers);
    }
}
