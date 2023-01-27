fn main() {
    println!("Hello, world! Yup");
    let mut number: i32 = 6;
    print!("{number}");
    while number != 1 {
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number = 3 * number + 1;
        }
        print!(" -> {number}");
    }
    println!();

    let text: &str = "hello";
    let message: &str = "
        The message is \n here
    ";
    println!("{text}");
    println!("{message}");
}
