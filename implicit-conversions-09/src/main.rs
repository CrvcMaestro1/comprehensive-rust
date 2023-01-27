fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;
    println!("{x} * {y} = {}", multiply(x.into(), y));
    println!("{x} * {y} = {}", multiply(i16::from(x), y));
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}