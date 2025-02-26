use std::env;
use std::process;
use regex::Regex;
use reqwest::blocking::{Client};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Serialize;

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

// Function to extract numbers from a string
fn extract_numbers(text: &str) -> Vec<u64> {
    let re = Regex::new(r"\b\d+\b").unwrap();  // Regular expression to match numbers
    re.find_iter(text)
        .filter_map(|mat| mat.as_str().parse::<u64>().ok())
        .collect()
}

// Struct to represent the request body for the GitHub API
#[derive(Serialize)]
struct CommentRequest {
    body: String,
}

fn post_github_comment(pr_number: u64, body: &str) -> Result<(), reqwest::Error> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");

    // GitHub API URL for posting comments
    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );

    let client = Client::new();

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);
    headers.insert("Accept", HeaderValue::from_static("application/vnd.github.v3+json"));

    // Prepare comment data
    let comment_data = CommentRequest { body: body.to_string() };

    // Send request to post comment
    client
        .post(&url)
        .headers(headers)
        .json(&comment_data)
        .send()?;

    Ok(())
}

fn main() {
    // Simulated PR content (this will be replaced by actual PR content in real cases)
    let pr_content = "Here are the numbers: 3, 5, and 8. Calculate Fibonacci for them.";
    println!("Simulated PR content: {}", pr_content);

    // Extract numbers from PR content
    let numbers = extract_numbers(pr_content);
    println!("Extracted numbers: {:?}", numbers);

    let (enable_fib, max_threshold) = parse_inputs();

    let mut comment_body = String::from("Fibonacci results:\n");

    if enable_fib {
        println!("Fibonacci calculation is enabled.");
        for &num in &numbers {
            if num <= max_threshold {
                let result = fibonacci(num);
                println!("Fibonacci of {}: {}", num, result);
                comment_body.push_str(&format!("Fibonacci of {}: {}\n", num, result));
            } else {
                println!("Number {} exceeds the max threshold", num);
                comment_body.push_str(&format!("Number {} exceeds the max threshold\n", num));
            }
        }
    } else {
        println!("Fibonacci calculation is disabled.");
        comment_body.push_str("Fibonacci calculation is disabled.\n");
    }

    // Get the pull request number from the environment (you can retrieve this dynamically in a real PR)
    let pr_number = env::var("GITHUB_REF_NAME").unwrap_or_else(|_| String::from("0")).parse::<u64>().unwrap_or(0);

    // Post the result to GitHub
    if let Err(e) = post_github_comment(pr_number, &comment_body) {
        eprintln!("Failed to post comment: {}", e);
        process::exit(1);
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
