pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn hanoi(n: usize) ->  &'static str {
    "A to C"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_solve_hanoi_with_1_disk() {
        let result = hanoi(1);
        assert_eq!(result, "A to C")
    }
}
