pub fn example_add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod example {
    use super::*;

    #[test]
    fn test_example_add() {
        assert_eq!(example_add(1, 1), 2);
    }
}