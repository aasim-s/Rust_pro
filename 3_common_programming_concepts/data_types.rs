fn main() {
    // 4 scalars ---> int, float, boo, char

    // unsigned integers ---> u8/16/32/64/128
    let x: u32 = 42;
    // signed integers ---> i8/16/32/64/128
    let y: i32 = -43;

    // floats are of two types f32/64
    let a = 2.0; // default f64
    let b: f32 = 4.3;

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotations

    // char --->4 bytes unicode scalar value, so its more than just ASCII
    let c = 'c';
    let d: char = 'D'; //with explicit type annotations
    let z: char = 'ℤ';
    let emoji = '😻';

    // Compound Types : can group multiple values in in type
    // 2 primitive compound types ---> tuple, array

    // tuple
    let tup = (220, 77, 5, 23);
    let tuple: (i32, f64, u8) = (500, 6.4, 1); // with explicit type annotations

    // use pattern matching to destructure a tuple or with .
    // let (x, y, z) = tup or tup.0 tup.1 etc...
    // tuple without any values is called unit and written as ()

    // array
    let nums = [1, 2, 3, 4, 5];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type and size annotations
}
