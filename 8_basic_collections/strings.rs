/* String is implemented as collection of bytes*/

/* Rust has only one string type in the core language, which is the string slice str that
* is usually seen in its borrowed form &str (UTF-8) */

/* The String type, which is provided by Rust’s standard library rather than coded
*  into the core language, is a growable, mutable, owned, UTF-8 encoded string type */

fn main() {
    //create new string
    let mut s = String::new(); // empty

    let data = "content";
    let mut s2 = data.to_string();
    let s3 = "content".to_string();

    let s4 = String::from("world");

    // updating a string
    s.push_str("hello");
    s2.push_str(&s3);
    // push_str method does not take ownership of s3 so we can still use it
    println!("s3 is {s3}");

    // use push for adding a char
    s.push(' ');

    // concatinate
    let s5 = s + &s4; // here s1 has been moved and can no longer be used;
    println!("{s5}");

    /* plus (+) operator calls the function fn add(self, s: &str) -> String
     *  thats why we used s1 direct and s4 as a ref*/

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");
    // to concatenate multiple strings use format instead of multiple plus(+)
    //let s9 = s6 + "-" + &s7 + "-" + &s8; // not neat
    let _s9 = format!("{s6}-{s7}-{s8}");
    // format uses ref so original strings are stll useable

    /* if you try to access parts of a String using indexing syntax in Rust, you’ll get an error
     *  A String is a wrapper over a Vec<u8>. utf-8 encoded element can be more than 1 byte each
     *  so it is not possible to get exact index value */

    // this string begins with the capital Cyrillic letter Ze, not the number 3
    let hello = String::from("Здравствуйте"); // russian
                                              // let first = &hello[0]; // wont compile if uncommented
                                              // When encoded in UTF-8, the first byte of З is 208 and the second is 151,
                                              // so it would seem that answer should in fact be 208, but 208 is not a valid character on its own

    // can use index but a range works but can be risky
    let s10 = &hello[0..4]; // output will be Зд
                            // let s10 = &hello[0..1]; // will cause runtime error

    // iterate over a string
    for _c in hello.chars() {
        // do something
    }

    for _b in s10.bytes() {
        // do something
    }
}
