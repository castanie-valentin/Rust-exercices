use std::io::{self, prelude::*};
use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};

fn main()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();

    let end_line = "***";

    let mut line = lines.next().expect("A line is needed");

    let mut list_vote: HashMap<String, i32> = HashMap::new();

    while line != end_line {
        let name = line.parse::<String>().unwrap();

        if list_vote.contains_key(line) {
            let value = list_vote.get(line).unwrap();
            list_vote.insert(name, value + 1);
        } else {
            list_vote.insert(name, 1);
        }

        line = lines.next().expect("A line is needed");
    }

    let (max_key, max_value) = list_vote.iter().max_by_key(|&(_, &count)| count).unwrap();

    let count_max_value = list_vote.values().filter(|&&value| &value == max_value).count();

    if count_max_value == 1 {
        println!("{}", max_key);
    } else {
        println!("Runoff!");
    }


}
