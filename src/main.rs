use std::env;
use std::process;
use fibonacci::{fibonacci, extract_numbers};

fn parse_inputs() -> (bool, u64) {
    let enable_fib = env::var("INPUT_ENABLE_FIB")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .unwrap_or(false);

    let max_threshold = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| String::from("1000"))  // Default is 1000
        .parse::<u64>()
        .unwrap_or(1000);  // Default is 1000

    (enable_fib, max_threshold)
}

// Main logic for processing PR content and Fibonacci calculation
fn main() {
    // Print "Hello, World!" for Day 2
    println!("Hello, World!");

    // Simulated PR content (this will be replaced by actual PR content in real cases)
    let pr_content = "Here are the numbers: 3, 5, and 8. Calculate Fibonacci for them.";
    println!("Simulated PR content: {}", pr_content);

    // Extract numbers from PR content
    let numbers = extract_numbers(pr_content);
    println!("Extracted numbers: {:?}", numbers);

    let (enable_fib, max_threshold) = parse_inputs();

    if enable_fib {
        println!("Fibonacci calculation is enabled.");
        for &num in &numbers {
            let result = fibonacci(num);
            println!("Fibonacci of {}: {}", num, result);
        }
    } else {
        println!("Fibonacci calculation is disabled.");
    }
}
