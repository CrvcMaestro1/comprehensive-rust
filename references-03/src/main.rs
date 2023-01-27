fn main() {
    /* References */
    let mut reference: i32 = 10;
    let ref_reference: &mut i32 = &mut reference;
    *ref_reference = 20;
    println!("reference: {reference}");

    /* Dangling references */
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
    }
    println!("ref_x: {ref_x}");
}
