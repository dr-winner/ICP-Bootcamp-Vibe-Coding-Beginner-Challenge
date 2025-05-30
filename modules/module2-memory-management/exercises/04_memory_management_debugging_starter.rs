// Uncomment each section one at a time and fix the issues

// Problem 1: Fix ownership errors
fn problem1() {
    // 1.1: Fix the double-move error
    let data = vec![1, 2, 3];
    let x = data.clone(); // Clone instead of move
    let y = data; // Now data can be moved
    println!("{:?} {:?}", x, y);

    // 1.2: Fix the ownership issue with the function
    let name = String::from("Rust");
    print_data(name.clone()); // Clone before passing
    println!("My name is {}", name); // Now name is still valid
}

fn print_data(data: String) {
    println!("Data: {}", data);
}

// Problem 2: Fix borrowing conflicts
fn problem2() {
    // 2.1: Fix the mutable/immutable borrow conflict
    let mut numbers = vec![1, 2, 3];
    let first = numbers[0]; // Store the value instead of a reference
    numbers.push(4);
    println!("First element is: {}", first);

    // 2.2: Fix the multiple mutable borrows
    let mut data = String::from("Hello");
    {
        let ref1 = &mut data;
        *ref1 = String::from("Hello, ");
    } // ref1 goes out of scope here
    {
        let ref2 = &mut data;
        *ref2 = ref2.to_string() + "Rust!";
    }
    println!("Data: {}", data);
}

// Problem 3: Fix dangling references
fn problem3() {
    // 3.1: Fix the dangling reference returned by the function
    let result = get_string();
    println!("Result: {}", result);

    // 3.2: Fix the issue with references outliving the data
    let reference;
    {
        let data = vec![1, 2, 3];
        reference = data; // Store the data instead of a reference
    }
    println!("Reference: {:?}", reference);
}

fn get_string() -> String { // Return String instead of &String
    String::from("I am no longer a dangling reference")
}

// Problem 4: Fix lifetime problems
fn problem4() {
    // 4.1: Fix the function signature to properly handle lifetimes
    let string1 = String::from("long string is long");
    let string2 = String::from("short"); // Move string2 outside the inner scope
    let result = longest(string1.as_str(), string2.as_str());
    println!("Longest string: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Problem 5: Optimize unnecessary cloning
fn problem5() {
    // 5.1: Remove unnecessary clones while keeping the code functional
    let original = String::from("Rust Programming");
    let len = calculate_length(&original); // Pass reference instead of cloning
    
    let mut names = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));
    
    for name in &names { // Use reference in the loop
        print_string(name); // Pass reference to print_string
    }
    
    println!("Original is still: {}", original);
    println!("Length was: {}", len);
    println!("Names: {:?}", names);
}

fn calculate_length(s: &String) -> usize { // Take reference instead of owned String
    s.len()
}

fn print_string(s: &String) { // Take reference instead of owned String
    println!("{}", s);
}

fn main() {
    println!("Running all fixed problems:");
    
    println!("\nProblem 1:");
    problem1();
    
    println!("\nProblem 2:");
    problem2();
    
    println!("\nProblem 3:");
    problem3();
    
    println!("\nProblem 4:");
    problem4();
    
    println!("\nProblem 5:");
    problem5();
}