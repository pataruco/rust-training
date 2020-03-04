fn main() {
    let numbers = 0..101;

    for number in numbers {
        match number {
            number if number % 5 == 0 && number % 3 == 0 => println!("{} fizzbuzz", number),
            number if number % 3 == 0 => println!("{} fizz", number),
            number if number % 5 == 0 => println!("{} buzz", number),
            _ => println!("{}", number),
        }
    }
}
