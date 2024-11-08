/* There are two ways to cause a panic in practice: by taking an action that causes our code to panic
* (such as accessing an array past the end) or by explicitly calling the panic! macro.
* */

/* By default, panics will print a failure message, unwind, clean up the stack, and quit.
* Via an environment variable, you can also have Rust display the call stack when a panic occurs
* to make it easier to track down the source of the panic.*/

/* By default, when a panic occurs the program starts unwinding, which means Rust walks back up
* the stack and cleans up the data from each function it encounters. However, walking back and cleaning up
* is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting,
* which ends the program without cleaning up.*/

/* Memory that the program was using will then need to be cleaned up by the operating system.
* If in your project you need to make the resultant binary as small as possible, you can switch
* from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile]
* sections in your Cargo.toml file. For example, if you want to abort on panic in release mode,
* add this:*/
// [profile.release]
// panic = 'abort'

fn main() {
    // manually induced panic
    //panic!("crash and burn");

    let _v = vec![1, 2, 3];
    //_v[5]; //panic due to bad code
}
