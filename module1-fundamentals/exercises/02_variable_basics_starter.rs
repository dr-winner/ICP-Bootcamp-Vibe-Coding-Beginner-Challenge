fn main() {
    // TODO: 1. Declare an immutable integer variable
    let my_integer = 50;

    // TODO: 2. Declare a mutable float variable and modify it later
    let mut my_float = 23.4;
    let original_float = my_float; // Store the original value for later use
    
    // TODO: Modify the float value
    my_float = 45.6;
    // TODO: 3. Declare a boolean variable using type inference
    let is_rust_fun = true;
    
    // TODO: 4. Declare a character variable with explicit type annotation
    let my_char: char = 'R';
    
    // TODO: 5. Perform arithmetic operations with the numeric variables
    let sum = my_integer + my_float as i32; // Convert float to integer for addition
    let product = my_float * my_integer as f64;
    
    // TODO: 6. Print all variables and calculation results with appropriate labels
    println!("Integer value: {}", my_integer);
    println!("Original float value: {}", original_float);
    println!("Modified float value: {}", my_float);
    println!("Boolean value: {}", is_rust_fun);
    println!("Character value: {}", my_char);
    println!("Addition result: {}", sum);
    println!("Multiplication result: {}", product);   
}