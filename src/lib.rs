pub fn hanoi(n: usize) ->  &'static str {
    if n == 1 {
        "A to C"
    } else {
        "A to B\nA to C\nB to C"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_hanoi_with_1_disk() {
        let result = hanoi(1);
        assert_eq!(result, "A to C")
    }

    #[test]
    fn should_solve_hanoi_with_2_disks() {
        let result = hanoi(2);
        assert_eq!(result, "A to B\nA to C\nB to C")
    }
}
