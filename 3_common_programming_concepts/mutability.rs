fn main() {
    // let x = 5;
    let mut x = 5;
    let y = 3;
    const FIXED_VALUE: u32 = 60 * 3;
    println!("The constant value is {FIXED_VALUE}");

    println!("The value of y before shadowing is {y}");

    let y = y + 4;
    println!("The value of y after shadowing is {y}");

    {
        let y = y + 2;
        println!("The value of y after shadowing in different scope is {y}");
    }

    println!("The value of y outside of scope is {y}");

    println!("The value of x is: {x}");
    x = 7; // x if is not mut so code will not compile
    println!("The new value of x is {x}");
}
