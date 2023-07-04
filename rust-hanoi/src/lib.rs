pub fn hanoi(n: usize) ->  Vec<String> {
    hanoi_iter(
        n,
        "A",
        "C",
        "B",
        Vec::new()
    )
}

fn hanoi_iter(n: usize, src: &str, dest: &str, aux: &str, mut acc: Vec<String>) -> Vec<String> {
    if n == 1 {
        acc.push(src.to_string() + " to " + dest);
        acc
    } else {
        acc = hanoi_iter(n - 1, src, aux, dest, acc);
        acc = hanoi_iter(1, src, dest, aux, acc);
        hanoi_iter(n - 1, aux, dest, src, acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_hanoi_with_1_disk() {
        let result = hanoi(1);
        assert_eq!(result, vec![String::from("A to C")])
    }

    #[test]
    fn should_solve_hanoi_with_2_disks() {
        let result = hanoi(2);
        assert_eq!(result, vec![
            String::from("A to B"),
            String::from("A to C"),
            String::from("B to C"),
        ])
    }

    #[test]
    fn should_solve_hanoi_with_3_disks() {
        let result = hanoi(3);
        assert_eq!(result, vec![
            String::from("A to C"),
            String::from("A to B"),
            String::from("C to B"),
            String::from("A to C"),
            String::from("B to A"),
            String::from("B to C"),
            String::from("A to C"),
        ])
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
