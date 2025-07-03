#[derive(Debug, PartialEq)]

enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u8),
}

fn fizz_buzz(n: u8) -> Vec<FizzBuzz> {
    let mut fizz_buzz_values: Vec<FizzBuzz> = vec![];

    for number in 1..=n {
        let fizz = number % 3 == 0;
        let buzz = number % 5 == 0;

        let value = match (fizz, buzz) {
            (true, true) => FizzBuzz::FizzBuzz,
            (true, false) => FizzBuzz::Fizz,
            (false, true) => FizzBuzz::Buzz,
            (false, false) => FizzBuzz::Number(number),
        };

        fizz_buzz_values.push(value);
    }

    fizz_buzz_values
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use crate::FizzBuzz;

    use super::fizz_buzz;

    #[test]
    fn test_fizz_buzz() {
        let result = fizz_buzz(1);
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn test_fizz_with_two_numbers() {
        let result = fizz_buzz(2);
        assert_eq!(result, vec![FizzBuzz::Number(1), FizzBuzz::Number(2)]);
    }
    #[test]
    fn test_fizz_with_three_and_fizz() {
        let result = fizz_buzz(3);
        assert_eq!(
            result,
            vec![FizzBuzz::Number(1), FizzBuzz::Number(2), FizzBuzz::Fizz]
        );
    }

    #[test]
    fn test_fizz_with_five_and_buzz() {
        let result = fizz_buzz(5);
        assert_eq!(
            result,
            vec![
                FizzBuzz::Number(1),
                FizzBuzz::Number(2),
                FizzBuzz::Fizz,
                FizzBuzz::Number(4),
                FizzBuzz::Buzz
            ]
        );
    }

    #[test]
    fn test_fizz_with_five_and_fizz() {
        let result = fizz_buzz(6);
        assert_eq!(
            result,
            vec![
                FizzBuzz::Number(1),
                FizzBuzz::Number(2),
                FizzBuzz::Fizz,
                FizzBuzz::Number(4),
                FizzBuzz::Buzz,
                FizzBuzz::Fizz
            ]
        );
    }
    #[test]
    fn test_fizz_with_10_and_buzz() {
        let result = fizz_buzz(10);
        assert_eq!(
            result,
            vec![
                FizzBuzz::Number(1),
                FizzBuzz::Number(2),
                FizzBuzz::Fizz,
                FizzBuzz::Number(4),
                FizzBuzz::Buzz,
                FizzBuzz::Fizz,
                FizzBuzz::Number(7),
                FizzBuzz::Number(8),
                FizzBuzz::Fizz,
                FizzBuzz::Buzz
            ]
        );
    }
    #[test]
    fn test_fizz_with_15_and_fizzbuzz() {
        let result = fizz_buzz(15);
        assert_eq!(
            result,
            vec![
                FizzBuzz::Number(1),
                FizzBuzz::Number(2),
                FizzBuzz::Fizz,
                FizzBuzz::Number(4),
                FizzBuzz::Buzz,
                FizzBuzz::Fizz,
                FizzBuzz::Number(7),
                FizzBuzz::Number(8),
                FizzBuzz::Fizz,
                FizzBuzz::Buzz,
                FizzBuzz::Number(11),
                FizzBuzz::Fizz,
                FizzBuzz::Number(13),
                FizzBuzz::Number(14),
                FizzBuzz::FizzBuzz
            ]
        );
    }
}
