use std::io::{self, prelude::*};
fn main() {
    let mut input =  String::new();

    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let lines = input.lines();

    let mut list : Vec<i32> = Vec::new();

    for (line_number, line) in lines.enumerate() {
        if line.contains("FBI") {
            list.push((line_number + 1) as i32);
        }
    }

    if list.is_empty(){
        println!("HE GOT AWAY!")
    } else {
        let formatted_numbers: String = list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", formatted_numbers);
    }

}
