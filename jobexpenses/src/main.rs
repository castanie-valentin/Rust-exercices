use std::io::{self, prelude::*};

fn main() {
    let mut input =  String::new();

    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let mut lines = input.lines();

    lines.next();

    let second_line = lines.next().expect("A second line");

    let expenses : Vec<i32> = second_line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("entier"))
        .collect();

    let total_expense: i32 = expenses.iter().filter(|&&x| x < 0).sum();

    println!("{}",-total_expense);

}
