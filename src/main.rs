use std::{io, cmp::Ordering};
use rand::Rng;

fn main() -> io::Result<()> {
    // Create random number generate and then pick a number between 1 and 100
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..101);

    // Create empty string and stdin object.
    // Then read the input of stdin and assign it to user_input
    let mut user_input = String::new();

    loop {

        let stdin = io::stdin();
        println!("Please type a number between 1 and 100: ");
        user_input.clear();
        stdin.read_line(&mut user_input)?;

        // Parse the number as i32 from the user input string
        let user_num = match user_input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number!");
                continue;
            }
        };
        match user_num.cmp(&random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You won!");
                break
            }
        }
    }
    Ok(())
}
