use std::env;
use regex::Regex;
use reqwest::blocking::{Client, Response};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Comment {
    body: String,
}

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

fn extract_numbers(text: &str) -> Vec<u64> {
    let re = Regex::new(r"\b\d+\b").unwrap();  // Regular expression to match numbers
    re.find_iter(text)
        .filter_map(|mat| mat.as_str().parse::<u64>().ok())
        .collect()
}

fn post_comment(pr_url: &str, body: &str, token: &str) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let comment = Comment { body: body.to_string() };

    client
        .post(format!("{}/comments", pr_url))
        .header(AUTHORIZATION, format!("token {}", token))
        .header(USER_AGENT, "FibBot Action")
        .json(&comment)
        .send()
}

fn main() {
    // Simulated PR content
    let pr_content = "Here are the numbers: 3, 5, and 8. Calculate Fibonacci for them.";
    let numbers = extract_numbers(pr_content);

    // Extract inputs
    let (enable_fib, max_threshold) = parse_inputs();

    if enable_fib {
        let mut result = String::from("Fibonacci calculation results:\n");
        for &num in &numbers {
            if num <= max_threshold {
                let fib_result = fibonacci(num);
                result.push_str(&format!("Fibonacci of {}: {}\n", num, fib_result));
            } else {
                result.push_str(&format!("Skipping number {} as it exceeds the max threshold of {}\n", num, max_threshold));
            }
        }

        // GitHub token and PR URL from environment variables
        let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set");
        let pr_url = env::var("PR_URL").expect("PR_URL is not set");

        // Post the result as a comment on the pull request
        match post_comment(&pr_url, &result, &github_token) {
            Ok(response) => println!("Posted comment: {:?}", response),
            Err(e) => eprintln!("Error posting comment: {}", e),
        }
    } else {
        println!("Fibonacci calculation is disabled.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_extract_numbers() {
        let pr_content = "The numbers are 3, 5, and 8.";
        let numbers = extract_numbers(pr_content);
        assert_eq!(numbers, vec![3, 5, 8]);
    }

    #[test]
    fn test_extract_numbers_with_non_numbers() {
        let pr_content = "This is a test with no numbers";
        let numbers = extract_numbers(pr_content);
        assert_eq!(numbers, vec![]);  // Should return an empty vector
    }
}
