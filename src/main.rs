use std::env;
use std::process;

fn parse_inputs() -> (bool, u64) {
    let enable_fib = env::var("INPUT_ENABLE_FIB")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .unwrap_or(false);

    let max_threshold = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| String::from("1000")) 
        .parse::<u64>()
        .unwrap_or(1000);  

    (enable_fib, max_threshold)
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

fn main() {
    
    println!("Hello, world!");

    let (enable_fib, max_threshold) = parse_inputs();

    if enable_fib {
        println!("Fibonacci calculation is enabled.");
        for i in 0..=max_threshold {
            let result = fibonacci(i);
            println!("Fibonacci of {}: {}", i, result);
        }
    } else {
        println!("Fibonacci calculation is disabled.");
    }
}
