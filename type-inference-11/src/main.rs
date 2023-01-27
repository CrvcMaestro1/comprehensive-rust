fn main() {
    let x = 10;
    let y = 20;
    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);

    let mut vector = Vec::new();
    vector.push((10, false));
    vector.push((20, true));
    println!("vector: {vector:?}");

    let vector_v2 = vector.iter().collect::<std::collections::HashSet<_>>();
    println!("vector V2: {vector_v2:?}");
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

