// Chapter 3 Section 3 Examples

fn main() {

    // Tuple
    let tup : (i32, u64, f32) = (120, 98765436521, 3.145159);
    
    // Unpack that tuple
    let (x, y, z) = tup;
    println!("x: {}   y: {}   z: {}", x, y, z);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let b: [u8; 4] = [0, 1, 2, 3];
    let c = [4; 3];

    // Access Array Elements
    println!("First element of a: {}", a[0]);
    println!("First element of b: {}", b[0]);
    println!("First element of c: {}", c[0]);



}
