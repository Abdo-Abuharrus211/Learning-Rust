use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter a number guess between 1  and 100.\nType 'q' to quit.");
    let rand_number = rand::rng().random_range(1..=100);
    loop {
        let mut guess_value = String::new();
        io::stdin()
            .read_line(&mut guess_value)
            .expect("Failed to read stdin line input");
        if guess_value.trim() == "q" {
            break;
        }
        let guess_value: u32 = match guess_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess_value.cmp(&rand_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You Win!\nThe number was {}", rand_number);
                break;
            }
        }
    }
}
