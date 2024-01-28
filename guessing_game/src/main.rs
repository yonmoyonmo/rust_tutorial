use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("UP & DOWN : guess a number between 1 and 100");
    let target = rand::thread_rng().gen_range(1..=100);
    loop{
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("You Entered: {}", input.trim());
                break;
            }
            Err(_) => {
                println!("Input failed, try again");
                continue;
            }
        };

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your guess : {input}");

        match target.cmp(&input) {
            Ordering::Less => println!("Up!"),
            Ordering::Greater => println!("Down"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
