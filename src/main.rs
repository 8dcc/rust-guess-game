use std::io;  // For input
use std::cmp::Ordering;  // For comparing the user's choice and the random number
use rand::Rng;  // For random number generation

fn main() {
    println!("Guess the number!");  // Print in a new line "Guess the number!"

    let secret_number = rand::thread_rng().gen_range(1..101);  // Create a variable called secret_number with a random integer in the range 1-101

    //println!("The secret number is: {}", secret_number);  // Print the number chosen (debug or cheating)

    loop {  // Start an infinite loop
        println!("Please input your guess.");  // Print in a new line "Please input your guess."

        let mut guess = String::new();  // Create a new mutable variable called guess. Type String

        io::stdin()                            // For reading the input.
            .read_line(&mut guess)             // Read the input and store it in guess. You can type this in the next line with a tab.
            .expect("Failed to read line");    // Type this and exit the program if the user types an incorrect value (Err)

        let guess: u32 = match guess.trim().parse() {    // Make a new u32 variable called guess; remove the spaces (and the \n); parse it into an integer;
            Ok(num) => num,                              // check if the value is "parsable" if it is, do nothing and return the value
            Err(_) => {                                  // If there is an error:
                print!("You did not type a number! ");   // Print without enter that the user did not type an integer.
                continue;                                // Ask for another value.
            }
        };

        match guess.cmp(&secret_number) {               // Compare the guess variable vs secret_number
            Ordering::Less => println!("Too small!"),   // If the secret_number is less than guess, print too small
            Ordering::Greater => println!("Too big!"),  // If the secret_number is more than guess, print too big
            Ordering::Equal => {                        // If the secret_number is the same as guess, user wins
                println!("You win!");
                break;
            }
        }
    }
}
