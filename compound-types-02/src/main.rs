fn main() {
    /* Compound types */

    /* Arrays */
    let mut a: [i8; 10] = [4; 10];
    let mut b: [i8; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    a[5] = 0;
    b[10] = 1;
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("{b:#?}");


    /* Tuples */
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}
