use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secrete_number = rand::thread_rng().gen_range(1..11);

    println!("GMN - Guess My Number");

    loop {
        println!("Type a number");

        let mut gmn = String::new();

        io::stdin().read_line(&mut gmn).expect( "Error reading file");

        // let gmn: u32 = gmn.trim().parse().expect("Type a number");
        let gmn: u32 = match gmn.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You type: {}", gmn);

        match gmn.cmp(&secrete_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Gotcha");
                break;
            }
        }
    }
}
