use std::io;

fn take_inputs(input_for: &mut String) {
    io::stdin()
        .read_line(input_for)
        .expect("Error reading line.");
}