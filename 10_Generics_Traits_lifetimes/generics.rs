// Struct using a generaic type
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// You cannot simultaneously implement specific and generic methods of the same name.

struct Point2<T, U> {
    x: T,
    y: U,
}

// Enum using a generic type
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// find largest in the list for i32
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// find largest in the list for char
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// find largest in the list using generic type
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    // let _mix = Point { x: 5, y: 4.0 }; // this will give error as two types
    let _mix = Point2 { x: 5, y: 4.0 };

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'a', 'm', 'l'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
    let result = largest(&char_list);
    println!("The largest char is {result}");
}
