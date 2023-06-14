fn main() {
    let mut input = String::new();
    let output: &str;

    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: usize = input
        .trim()
        .parse()
        .expect("Input entered was not a number");

    output = if number % 2 == 0 {"Number is even"} else {"Number is odd"};

    println!("{output}")
}