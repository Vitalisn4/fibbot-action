use regex::Regex;

pub fn extract_numbers(text: &str) -> Vec<u64> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(text)
        .filter_map(|mat| mat.as_str().parse::<u64>().ok())
        .collect()
}

pub fn fibonacci(n: u64) -> u64 {
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
