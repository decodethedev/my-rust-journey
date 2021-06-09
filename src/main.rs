use std::io; // use the standard library, kinda like c++ huh
use std::cmp::Ordering;
use rand::Rng; //use the random librabry
fn /*Declare a function*/ main() /*Parameters here kinda like c++*/ {


    println!("Guessing Game By Decode!"); // this is how we prints in Rust
    let secret_number = rand::thread_rng().gen_range(1..101); // create a variable and get the random number from 1 to 101 this means numbers from 1 to 100 not counted 101
    /*
        Alternatively:
        rand::thread_rng().gen_range(1..=100)
    */
    loop {
        
        println!("Please give us your guess!");


        let mut user_guess = String::new(); // String::new() creates a new string instance and put it in userGuess
        /*
        let declares a variable
        mut means mutable
        if the let statement with no mut means they are immutable

        *NOTE* You also cant make snake strings such as helloWorld you cant do that because it will raise 'snake case' error 
        */


        io::stdin() // calls the stdin function from io
            .read_line(&mut user_guess) // read the current line and put the inputs inside userGuess
            .expect("Please give a guess!"); // catch errors if there is an error!

        //let user_guess: u32 = user_guess.trim().parse().expect("Please type a number!");
        // BETTER SOLUTION:
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num, // if success then nothing happens
            Err(_) => continue, // if raised errors then jumps back to the start of the loop to ask for a new input
        };
        /* The : after the var indicates what type of variable we want to use u32 is unsigned 32-bit int*/

        // Checks if guess is a number if no then Warns the user
        
        // trim() removes the \n because when we do read_line the user will press enter which creates '\n' so trim removes it

        // parse() parses a string into a number but sometimes it might cause errors so we might want to put .expect to catch errors incase user put some special characters
        
        println!("{}", format!("Your guess - >{}",user_guess)); // printing the function out, we cant put the variable naked we must put a pair of curly brackets representing where the variable is located
        /* 
        format!("Hello");                 // => "Hello"
        format!("Hello, {}!", "world");   // => "Hello, world!"
        format!("The number is {}", 1);   // => "The number is 1"
        format!("{:?}", (3, 4));          // => "(3, 4)"
        format!("{value}", value=4);      // => "4"
        format!("{} {}", 1, 2);           // => "1 2"
        format!("{:04}", 42);             // => "0042" with leading zeros
        */


        match user_guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            } 
        }
        
    };
    println!("{}", format!("Computer's guess => {}",secret_number));
}
