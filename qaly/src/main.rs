use std::io::{self, prelude::*};

fn main() {
    let mut input =  String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();

    let first_line = lines.next().expect("A line is needed");
    let number_of_period = first_line.parse::<usize>().unwrap();

    let mut quality : Vec<f64> = Vec::new();
    let mut period : Vec<f64> = Vec::new();

    for line in lines {
        let mut iterateur = line.split_whitespace();

        let quality_number = iterateur.next();
        quality.push(quality_number.unwrap().parse::<f64>().unwrap());

        let period_number = iterateur.next();
        period.push(period_number.unwrap().parse::<f64>().unwrap());
    }

    let mut res : f64 = 0.0;

    for n in 0..number_of_period{
        res = res + quality[n] * period[n];
    }

    println!("{:.3}", res);
}
