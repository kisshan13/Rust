use std::io;

fn calculator() {
    let mut input_one = String::from("");
    let mut input_two = String::from("");

    println!("Enter number one: ");
    io::stdin()
        .read_line(&mut input_one)
        .expect("Can't read line");

    println!("Enter number two: ");
    io::stdin()
        .read_line(&mut input_two)
        .expect("Can't read line");

    let number1: i32 = input_one.trim().parse().expect("Can't do conversion");
    let number2: i32 = input_two.trim().parse().expect("Can't do conversion");

    let mut operation = String::from("");

    println!("Do: (Add, Subtract, Multiply, Divide) ");
    io::stdin().read_line(&mut  operation).expect("Can't read line");

    let mut check = true;

    match operation.trim().as_ref() {
        "Add" => {
            println!("Add {number1} + {number2} = {}", number1 + number2);
        },

        "Subtract" => {
            println!("Subract {number1} - {number2} = {}", number1 - number2);
        },

        "Multiply" => {
            println!("Multiply {number1} X {number2} = {}", number1 * number2);
        },

        "Divide" => {
            println!("Divide {number1} / {number2} = {}", number1 / number2)
        },

        _ => {
            println!("{operation}");
            println!("Sorry that's not a valid operation");
        }

    }
}
