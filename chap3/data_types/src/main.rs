fn main() {

    //   ---   Scalar Types   ---  //

    // Integer Types
    let a: u8 = 5;
    let b: u32 = 654321;
    let c: isize = -789456;
    
    // Integer Literals
    let decimal = 69_420;
    let hex = 0xdd;
    let oct = 0o77;
    let binary = 0b111_1010;

    // Foating-Point Numbers
    let a: f64 = 1.0;

    // Basic Mathematics Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4.0 * 56.5;
    let quotient = 987.654 / 654.321;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    // Boolean Data-Type
    let t: bool = true;
    let f: bool = false;

    // Characters
    let c = 'z';
    let z: char = '\u{03C0}';
    println!("pi = {z}");

    //   ---   Compund Types   ---  //

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup: x = {x}");
    println!("tup: y = {y}");
    println!("tup: z = {z}");

    // Arrays
    let x = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let q: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("Array: Index 0 yields {}", q[0]);
    println!("Array: Index 1 yields {}", q[1]);
    println!("Array: Index 2 yields {}", q[2]);
    println!("Array: Index 3 yields {}", q[3]);
    println!("Array: Index 4 yields {}", q[4]);

    // Invalid Array Access
    // println!("q[10] = {}", q[10]);


}
