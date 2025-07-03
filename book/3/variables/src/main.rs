// fn main() {
//     // addition
//     let sum = 5 + 10;
//     println!("sum 5 + 10 --> { }", sum);

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("difference 95.5 - 4.3 --> { }", difference);

//     // multiplication
//     let product = 4 * 30;
//     println!("product 4 * 30 --> { }", product);

//     // division
//     let quotient = 56.7 / 32.2;
//     println!("quotient 56.7 / 32.2 --> { }", quotient);

//     let truncated = -5 / 3; // Results in -1
//     println!("truncated -5 / 3; --> { }", truncated);

//     // remainder
//     let remainder = 43 % 5;
//     println!("remainder  43 % 5 --> { }", remainder);
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
