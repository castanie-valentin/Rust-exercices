use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let n  = input.trim().parse::<i32>().expect("Entier");

    if n <= 100 && n >= 0 {
        for i in 1..=n {
            println!("{} Abracadabra", i);
        }
    }
    
}
