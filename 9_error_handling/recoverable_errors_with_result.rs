/* Result enum is defined as
* enum Result<T, E> {
*       Ok(T),
*       Err(E),
* }
* */

/* like the Option enum, the Result enum and its variants have been brought into scope by the prelude,
* so we don’t need to specify Result:: before the Ok and Err variants in the match arms.*/

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let open_non_existing_file = File::open("test.txt");

    let _file = match open_non_existing_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening file the file: {other_error:?}");
            }
        },
    };

    // using closure(chapter 13) instead of match
    let _file2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // Shortcuts for Panic on Error
    let _file3 = File::open("hello.txt").unwrap();
    // if Result is Ok then value is retured if Result is err then panic is called
    let _file4 = File::open("hello.txt").except("hello.txt should be included in the project");
    // except is same as unwrap just that it lets you write custom panic message
}

// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
