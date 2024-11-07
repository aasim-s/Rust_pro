/*vectors allow you to store more than one value (of same type) in a single data structure that puts all
* the values next to each other in memory.*/

// when a vector is dropped all of its content also gets dropped

use std::ops::Range;
use std::slice::Iter;

fn main() {
    // type mentioned bcoz initialized without data
    // rust needs type at compile time to allocate required memory
    let _v1: Vec<i32> = Vec::new();
    // type need not be mentionied when initialized with data
    let v2 = vec![1, 2, 3]; //type is infered from the value

    // updating a vector
    let mut v3 = Vec::new();
    v3.push(5); //this will set the type at compile time, so its okay
    v3.push(7);
    v3.pop();

    // reading values
    let _third = &v2[2];
    // better way to do it
    let _second: Option<&i32> = v2.get(1);

    // iterating over a vector
    let v4 = vec![100, 32, 27];
    for n_ref in &v4 {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    // using an iterator on vector
    let mut v5: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v5.iter(); // iterator gives a pointer Option
    let _n1: &i32 = iter.next().unwrap();
    let _n2: &i32 = iter.next().unwrap();
    let _end: Option<&i32> = iter.next();

    // iterate without pointer
    let mut iter: Range<usize> = 0..v5.len();
    let i1: usize = iter.next().unwrap();
    let _n1: &i32 = &v5[i1];

    // it is inconvenient to just store values of same type only
    // can over come this using enum with vector
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(33.33),
    ];

    let mut k = vec![1, 2, 3];
    let mut k2: Vec<&mut i32> = Vec::new();
    for i in &mut k {
        k2.push(i);
    }
    *k2[0] = 5;
    let a = *k2[0];
    let b = k[0];
    println!("{a} {b}");
}
