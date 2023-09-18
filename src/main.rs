mod string;

pub use string::add_strings;

fn main() {
    println!("Hello, world!");

    println!("Added = {}", add(12, 5));
    println!("Multiple = {}", multiple(928347239, 324873));

    println!("Added strings, {}", add_strings("Hello", "Fuubar"));

    let result = minus(34, 34897);
    match result {
        Some(val) => println!("Minus : {}", val),
        None => println!("Minus resulted in underflow"),
    }

    let test2 = minus(100, 45);
    match test2 {
        Some(val) => println!("Minus : {}", val),
        None => println!("Minus resulted in underflow"),
    }

    let divided = divide(34, 912);
    println!("Diveded: {}", divided);
}

fn add(num_one: u64, num_two: u64) -> u64 {
    return num_one + num_two;
}

fn divide(num_one: u64, num_two: u64) -> u64 {
    return num_one / num_two;
}
fn minus(num_one: u64, num_two: u64) -> Option<u64> {
    return num_one.checked_sub(num_two);
}

fn multiple(num_one: u64, num_two: u64) -> u64 {
    return num_one * num_two;
}
