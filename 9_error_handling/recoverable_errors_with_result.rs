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
}
