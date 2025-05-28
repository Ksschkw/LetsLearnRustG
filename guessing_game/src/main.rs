// fn main() {
//     println!("Hello, world!");
// }
// Imports neccessar libs
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guessing Game");
    println!("--------------------------------------------------------------------------------------------------------------------");
    //using a loop to kep the game going
    loop{
        // generating random numbers from range 1 to 100 (1 and 100 included)
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Enter your guessed number negro");

        // creating a mutable variable for 'guess' because bby default, variables in rust are immutable
        let mut guess = String::new();
    
        //using the io library to read in input from user and feed it into the 'guess' variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");

        // trimmng and parsing the value in the 'guess' variable also handling the error of when parse fails to convert the input to a number 
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your guessed number is {}", guess);
    
        println!("The secret number was {secret_number}");
    
    
        //comparing the 'guess' input to the 'secret_number' and giving output
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small bitch\n--------------------------------------------------------------------------------------------------------------------"),
            Ordering::Greater => println!("Too big negro\n--------------------------------------------------------------------------------------------------------------------"),
            Ordering::Equal => {
                println!("You win!(Shocker!!)\n--------------------------------------------------------------------------------------------------------------------");
                //breaking the loop when the guess matches the secret_number
                break;
            }
        }
    }
}