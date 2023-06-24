use std::io;

fn is_prime() {
    let mut user_input = String::from("");

    println!("Please enter a number to check wheather it's a prime or not:");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Can't read line"
    );

    let number: i32 = user_input.trim().parse().expect("Can't do conversion");
    let mut is_prime: bool = true;

    for i in (2..number) {
        if number % i == 0 {
            is_prime = false;
            break;
        }
    }

    if is_prime {
        println!("{} is prime", number);
    }

    else {
        println!("{} is not prime", number)
    }
}