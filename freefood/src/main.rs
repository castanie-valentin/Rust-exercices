
use std::io::{self, prelude::*};
use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};
fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).expect("Line is expect");

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();

    let number_of_line = first_line.parse::<i32>().unwrap();

    let mut final_value: HashSet<i32> = HashSet::new();

    for n in 0..number_of_line {

        let mut next_line = lines.next().expect("test");

        let mut iterateur =  next_line.split_whitespace();

        let first_value = iterateur.next().unwrap().parse::<i32>().unwrap();

        let second_value = iterateur.next().unwrap().parse::<i32>().unwrap();

        for m in first_value..second_value + 1 {
            final_value.insert(m);
        }

    }

    let count = final_value.len();

    println!("{:?}", count);
}
