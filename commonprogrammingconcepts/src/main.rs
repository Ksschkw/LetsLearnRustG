use std::io;
fn main() {
    println!("VARIABLES--------------------------------------------------------------------------------------------------------------------");
    //VARIABLES
    let mut x = 5; //mutable
    // let x =  5; // immutable
    println!("The value of x is {x}");
    x = 7; 
    println!("The value of x is {x}");

    // CONSTANTS
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
    //Constants are different  from variables g, you can't use 'mut with them to make them mutable, they'll always remain immutable
    println!("CONSTANTS--------------------------------------------------------------------------------------------------------------------");
    const THREE_HOURS_IN_SECONDS: u32 =  60*60*3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // i am just going to try something
    println!("TIMECONVERSION--------------------------------------------------------------------------------------------------------------------");
    println!("Enter a number(in hours) to convert to minutes and seconds: \nOr enter any key to quit the whole program(You do not want to do that). I'd explain the reason, but that's too long, just do as i say like a good girl");
    let mut input= String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    let input: u32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => // break,
        {
            println!("Why did you quit on me g ?, why ?😥😣\n DAMN!!!!💔\n--------------------------------------------------------------------------------------------------------------------");
            return; //Exit if parsing fail
            // break;
        },
    };
    let input_hours_in_seconds = 60*60*input;
    let input_hours_in_minutes = 60*input;
    let input_hours_in_days: f64 = input as f64/24.0;
    println!("{input} hours is {input_hours_in_minutes}");
    println!("{input} hours is {input_hours_in_seconds} seconds");
    println!("{input} hours is {:.3} day/s\n--------------------------------------------------------------------------------------------------------------------",input_hours_in_days);
    //Finished

    //SHADOWS --Second/last overshadows the first
    //Example
    println!("SHADOWS--------------------------------------------------------------------------------------------------------------------");
    let x = 5;
    let x = x+1;
    {
        let x = x*2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x still remains {x}");

}
