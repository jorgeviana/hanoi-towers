pub fn hanoi(n: usize) ->  String {
    hanoi_iter(
        n,
        "A",
        "C",
        "B",
        "".to_owned()
    )
}

fn hanoi_iter(n: usize, src: &str, dest: &str, aux: &str, acc: String) -> String {
    if n == 1 {
        acc + &src + " to " + &dest + "\n"
    } else {
        let first = hanoi_iter(n - 1, src, aux, dest, acc);
        let second = hanoi_iter(1, src, dest, aux, first);
        hanoi_iter(n - 1, aux, dest, src, second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_hanoi_with_1_disk() {
        let result = hanoi(1);
        assert_eq!(result, "A to C\n")
    }

    #[test]
    fn should_solve_hanoi_with_2_disks() {
        let result = hanoi(2);
        assert_eq!(result, "A to B\nA to C\nB to C\n")
    }

    #[test]
    fn should_solve_hanoi_with_3_disks() {
        let result = hanoi(3);
        assert_eq!(result, "A to C\nA to B\nC to B\nA to C\nB to A\nB to C\nA to C\n")
    }
}

#[cfg(test)]
mod list_learning_tests {

    #[test]
    fn learn1() {
        let mut strings: Vec<String> = Vec::new();

        strings.push(String::from("Hello"));

        assert_eq!(strings, vec![String::from("Hello")])
    }

    fn l(mut strings: Vec<String>) -> Vec<String> {
        strings.push(String::from("Hello World"));
        strings.clone()
    }

    #[test]
    fn learn2() {
        let m: Vec<String> = Vec::new();
        let i = l(m);
        assert_eq!(i, vec![String::from("Hello World")])
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
