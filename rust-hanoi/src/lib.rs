pub fn hanoi(n: usize) ->  String {
    if n == 1 {
        "A to C".to_owned()
    } else {
        "A to B\nA to C\nB to C".to_owned()
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

#[cfg(test)]
mod string_learning_tests {

    #[test]
    fn learn1() {
        let a: &str = "a";
        let b: &str = "b";
        let result: String = a.to_owned() + b;
        assert_eq!(result, "ab")
    }

    #[test]
    fn learn2() {
        let a: &str = "a";
        let b: &str = "b";
        let result: &str = &(a.to_owned() + b);
        assert_eq!(result, "ab")
    }

    #[test]
    fn learn3() {
        let a: String = "a".to_owned();
        let b: String = "b".to_owned();
        let result: String = a + &b;
        assert_eq!(result, "ab")
    }
}
