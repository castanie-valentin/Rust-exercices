use std::io::{self, prelude::*};
use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};

fn main() {
    let mut input =  String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();

    let first_line = lines.next().expect("A line is needed");

    let number_of_cases = first_line.parse::<i32>().unwrap();

    for i in 0..number_of_cases {

        let number_cities = lines.next().expect("A line is needed");
        let mut list_city : HashSet<String> = HashSet::new();


        for city in 0..number_cities.parse::<i32>().unwrap() {

            let city_name = lines.next().expect("A line is needed");

            let city_n = city_name.parse::<String>().unwrap();

            list_city.insert(city_n);

        }

        println!("{:?}",list_city.len());

    }
}
