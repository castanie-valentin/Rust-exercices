use std::io::{self, prelude::*};

fn main() {

    let mut input = String::new();

    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");

    let  line = input.lines().next().expect("A line is needed"); 

   let mut iterateur  = line.split_whitespace();
   
   let first_value = iterateur.next().expect("first").parse::<i32>().expect("Entier");
    
   let second_value = iterateur.next().expect("second").parse::<i32>().expect("Entier");

   let res;

   if first_value == second_value && first_value != 0  {
        res = first_value + second_value;
        println!("Even {}", res)
   } else if first_value == 0 && second_value == 0 {
        println!("Not a moose")
   }else{
        if first_value < second_value {
            res = second_value * 2;
            println!("Odd {}", res)
        }else{
            res = first_value * 2;
            println!("Odd {}", res)
        }
   }
}
