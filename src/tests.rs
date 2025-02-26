use super::{fibonacci, extract_numbers};

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
        assert_eq!(numbers, vec![]); 
