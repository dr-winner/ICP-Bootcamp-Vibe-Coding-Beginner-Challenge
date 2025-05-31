// TODO: 1. Define a function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    // TODO: Return the sum of a and b
    return a + b;
}

// TODO: 2. Define a function that calculates the area of a rectangle
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    // TODO: Return the area (width × height)
    return width * height;
}

// TODO: 3. Define a function that checks if a number is prime
fn is_prime(number: u32) -> bool {
    // TODO: Implement the prime number check logic
    if number % 2 == 0 && number > 2 {
        return false; // Exclude even numbers greater than 2
    }
    for i in 3..=(number as f64).sqrt() as u32 {
        if number % i == 0 {
            return false;
        }
    }
        number > 1 // 1 is not prime
    }

// TODO: 4. Define a function that converts Fahrenheit to Celsius
// Formula: C = (F - 32) * 5/9
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // TODO: Implement the temperature conversion logic
    (fahrenheit - 32.0) * 5.0 / 9.0    
}

fn main() {
    // TODO: Call the addition function with different values and print the results
    let sum1 = add(10, 20);
    let sum2 = add(7, 15);
    
    // TODO: Calculate and print the area of rectangles with different dimensions
    let area1 = calculate_rectangle_area(40.0, 20.0);
    let area2 = calculate_rectangle_area(60.0, 10.0);
    
    // TODO: Test your prime number checker with several numbers
    let prime_check1 = is_prime(11);
    let prime_check2 = is_prime(8);
    
    // TODO: Convert and print some temperatures from Fahrenheit to Celsius
    let celsius1 = fahrenheit_to_celsius(100.0);
    let celsius2 = fahrenheit_to_celsius(61.0);
    
    // TODO: Print all results with appropriate labels
    println!("Sum of a and b is: {}", sum1);
    println!("Sum of a and b is: {}", sum2);
    println!("Area of rectangle with width 40.0 and height 20.0 is: {}", area1);
    println!("Area of rectangle with width 60.0 and height 10.0 is: {}", area2);
    println!("Is 11 a prime number? {}", prime_check1);
    println!("Is 8 a prime number? {}", prime_check2);
    println!("100°F is equivalent to {}°C", celsius1);
    println!("61°F is equivalent to {}°C", celsius2);
    
  
}