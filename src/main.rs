use std::io;

fn take_inputs(input_for: &mut String) {
    io::stdin()
        .read_line(input_for)
        .expect("Error reading line.");
}

fn main() {
    let mut input= String::from("");

    println!("Enter the number you want factorial for: ");
    take_inputs(&mut input);

    let mut factorial_for: u32 = input.trim().parse().expect("Can't parse the input invalid type");
    let mut factorial = 1;

    while factorial_for != 1 {
        factorial = factorial * factorial_for;
        factorial_for -= 1;
    }

    println!("{}", factorial)
}





