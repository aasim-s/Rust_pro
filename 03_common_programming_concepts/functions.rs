fn main() {
    println!("This is main function.");
    another_function();
    function_with_input(5);
    let x = function_with_return_value();
    println!("value returned from third function is {x}");
    let sum = plus_num(4, 5);
    println!("the sum of 4 and 5 is {sum}");
}

fn another_function() {
    println!("This is another function.");
}

fn function_with_input(x: u32) {
    println!("the input to the second functon is {x}")
}

fn function_with_return_value() -> i32 {
    4 // returns dont have semicolons
}

fn plus_num(num1: u32, num2: u32) -> u32 {
    num1 + num2
}
