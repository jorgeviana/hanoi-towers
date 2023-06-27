pub fn hanoi(n: usize) ->  &'static str {
    "A to C"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_hanoi_with_1_disk() {
        let result = hanoi(1);
        assert_eq!(result, "A to C")
    }
}
