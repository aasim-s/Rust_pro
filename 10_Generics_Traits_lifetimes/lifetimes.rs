/*
* every reference in Rust has a lifetime, which is the scope for which that reference is valid.
* Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
*
* We must annotate types only when multiple types are possible. In a similar way, we must annotate
* lifetimes when the lifetimes of references could be related in a few different ways.
*
* The main aim of lifetimes is to prevent dangling references, which cause a program to reference data
* other than the data it’s intended to reference.
* */

fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r); // r is a dangining reference as x does not live long enough

    let y = 6;
    let s = &y;
    println!("s: {s}"); // correct use but is limiting

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}")
}

/*fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/
// will give error:
// this function's return type contains a borrowed value, but the signature does not say
// whether it is borrowed from `x` or `y'.

/*
* Lifetime annotations don’t change how long any of the references live. Rather, they describe
* the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
* Just as functions can accept any type when the signature specifies a generic type parameter, functions
* can accept references with any lifetime by specifying a generic lifetime parameter.
* */

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit
// annotations. The first rule applies to input lifetimes, and the second and third rules apply to output
// lifetimes. If the compiler gets to the end of the three rules and there are still references for which
// it can’t figure out lifetimes, the compiler will stop with an error.

// These rules apply to fn definitions as well as impl blocks.

// The first rule is that the compiler assigns a different lifetime parameter to each lifetime in each
// input type.
// The function fn foo(x: &i32) would get one lifetime parameter and become fn foo<'a>(x: &'a i32).
// The function fn foo(x: &i32, y: &i32) would get two lifetime parameters and become
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32).
// The function fn foo(x: &ImportantExcerpt) would get two lifetime parameters and become
// fn foo<'a, 'b>(x: &'a ImportantExcerpt<'b>).

// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is
// assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

// The third rule is that, if there are multiple input lifetime parameters, but one of them is
// &self or &mut self because this is a method, the lifetime of self is assigned to all output
// lifetime parameters.

// One special lifetime is 'static, which denotes that the affected reference can live for the entire
// duration of the program. All string literals have the 'static lifetime,
// which we can annotate as follows:
const a: &'static str = "I have a static lifetime.";
