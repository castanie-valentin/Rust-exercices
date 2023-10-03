use std::io::{self, prelude::*};

fn main() {
    let mut input =  String::new();

    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let mut lines = input.lines();

    lines.next();

    let second_line = lines.next().expect("A second line");

    let temperatures: Vec<i32> = second_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let count_below_zero = temperatures.iter().filter(|&&t| t < 0).count();

    println!("{}", count_below_zero);

}
