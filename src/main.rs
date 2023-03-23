use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secgnm = rand::thread_rng().gen_range(1..11);

    println!("GMN - Guess My Number");

    loop {
        println!("Type a number");

        let mut gmn = String::new();

        io::stdin()
            .read_line(&mut gmn)
            .expect("Error reading number");

        let gmn: u32 = gmn.trim().parse().expect("Type a number");

        println!("You type: {}", gmn);

        match gmn.cmp(&secgnm) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Gotcha");
                break;
            }
        }
    }
}
