fn main() {
    // let array = [10, 20, 30];
    // print!("Iterating over array:");
    // for n in array {
    //     print!(" {n}");
    // }
    // println!();
    //
    // print!("Iterating over range:");
    // for i in 0..3 {
    //     print!(" {}", array[i]);
    // }
    // println!();
    let matrix = [
        [101, 102, 103], // --> the comment makes `rust-fmt` add a new line
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    return result;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}");
    }
}

