fn main() {
    /* Functions */
    fizzbuzz_to(20); // Defined below, no forward declaration needed
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false; // Corner case, early return
    }
    lhs % rhs == 0 // The last expression in a block is the return value
}

fn fizzbuzz(n: u32) -> () { // No return value means returning the unit type `()`
    let by_3: bool = is_divisible_by(n, 3);
    let by_5: bool = is_divisible_by(n, 5);
    let by_35: (bool, bool) = (by_3, by_5);
    match by_35 {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn fizzbuzz_to(n: u32) { // `-> ()` is normally omitted
    for i in 1..=n { // `=` represent less than equal to `n`
        fizzbuzz(i);
    }
}