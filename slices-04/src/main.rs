fn main() {
    /* Slices */
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    // a[3] = 0;
    let a_len: usize = a.len();
    println!("a size: {a_len:?}");

    let s: &[i32] = &a[2..4]; // slice from index 2 to 4
    let all_slice: &[i32] = &a[..]; // slice all array

    println!("s: {s:?}");
    println!("all slice: {all_slice:?}")
}
