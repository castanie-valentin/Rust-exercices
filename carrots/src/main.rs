use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let mut lines = input.lines();  

    let first_line = lines.next().expect("A line is needed");

   let mut iterateur  = first_line.split_whitespace();

   iterateur.next();

   let value = iterateur.next().expect("premiere");


   println!("{}", value);


   
}
