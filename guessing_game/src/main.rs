// fn main() {
//     println!("Hello, world!");
// }
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guessing Game");
    println!("----------------");
    loop{
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Enter your guessed number negro");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
    
        println!("your guessed number is {}", guess);
    
        println!("The secret number is {secret_number}");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small bitch"),
            Ordering::Greater => println!("Too big negro"),
            Ordering::Equal => {
                println!("You win!(Shocker!!)");
                break;
            }
        }
    }
}