use std::io;

fn main() {
    let mut input = String::new();
    let mut temperature_input = String::new();

    println!("Convert to [C] or [F]: ");
    io::stdin().read_line(&mut input).expect("Cannot read line");
    let convert: char = input.trim().parse().expect("Only expect 'C' and 'F'");

    println!("Temperature: ");
    take_input(&mut temperature_input);
    let temperature: i32 = temperature_input
        .trim()
        .parse()
        .expect("Only numeric values.");

    println!("{}", convert_to(convert, temperature));
}

fn convert_to(to: char, temp: i32) -> i32 {
    let converted_temp = if to == 'C' {
        (temp * 9 / 5) + 32
    } else {
        (temp - 32) / (9 / 5)
    };

    converted_temp
}

fn take_input(inpt: &mut String) {
    io::stdin().read_line(inpt).expect("Failed taking input");
}