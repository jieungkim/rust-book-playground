/*
 *   Copyright (c) 2020 
 *   All rights reserved.
 */
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("the random number {}", secret_number);


    loop {
        println!("type the number that you think correct.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("cannot read your input");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("input value: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("the input number is less than"); }
            Ordering::Greater => { println!("the input number ie greater than"); }
            Ordering::Equal => { println!("the input number is equal to"); break; }
        }
    }
}
