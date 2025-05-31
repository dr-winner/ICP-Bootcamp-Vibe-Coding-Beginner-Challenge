use std::io;

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    // TODO: Implement the FizzBuzz algorithm for numbers 1 to 20
    for i in 1..=20 {
        // TODO: Check if i is divisible by both 3 and 5
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        }
        // TODO: Check if i is divisible by 3
        else if i % 3 == 0 {
            println!("Fizz");
        }
        // TODO: Check if i is divisible by 5
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    // TODO: Create a variable to control the calculator loop
    let mut running = true;
    
    // TODO: Implement the calculator loop
    while running {
        // TODO: Show the menu options
        println!("Choose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        // Get the user's choice
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        // Convert choice to a number (with error handling)
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // TODO: Exit if the user chose option 5
        if choice == 5 {
            // TODO: Set running to false to exit the loop
            running = false;
            continue;
        }
        
        // TODO: Get the two input numbers from the user
        // First number
        // TODO: Read first number
        println!("Enter the first number: ");
        let mut first_number = String::new();
        io::stdin()
            .read_line(&mut first_number)
            .expect("Failed to read line");
        let first_number: f64 = first_number.trim().parse()
            .expect("Please type a number!");

        // Second number
        // TODO: Read second number
        println!("Enter the second number: ");
        let mut second_number = String::new();
        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read line");
        let second_number: f64 = second_number.trim().parse()
            .expect("Please type a number!");
        
        // TODO: Perform the selected operation using match or if statements
        match choice {
            1 => println!("{} + {} = {}", first_number, second_number, first_number + second_number),
            2 => println!("{} - {} = {}", first_number, second_number, first_number - second_number),
            3 => println!("{} * {} = {}", first_number, second_number, first_number * second_number),
            4 => {
                if second_number == 0.0 {
                    println!("Error: Cannot divide by zero!");
                } else {
                    println!("{} / {} = {}", first_number, second_number, first_number / second_number);
                }
            },
            _ => println!("Invalid option. Please try again."),
        }

        // Ask if the user wants to perform another calculation
        println!("Do you want to perform another calculation? (y/n): ");
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
            
        if response.trim().to_lowercase() == "n" {
            running = false;
        }
    }
    
    println!("Thank you for using the calculator!");
}