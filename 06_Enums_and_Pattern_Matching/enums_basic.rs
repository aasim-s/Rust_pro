// basic enum syntax
enum IpAddrType {
    V4,
    V6,
}
// let four = IpAddrType::V4;

//using enum in a function
fn route(_ip_kind: IpAddrType) {}

// using enum with struct
struct IpAddr {
    kind: IpAddrType,
    address: String,
}

// here it will be more consise to use enum directly instead of in a strut
// can directly put value in enum field
enum IpAddrEnum {
    V4(String),
    V6(String),
}

// another better way with enum
enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//bit more complicated enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// can defined methods on enum just like structs
impl Message {
    fn call(&self) {
        // some functionality
    }
}

// there is no null in rust but insead an enum called Option
// enum Option<T> {
//      None,
//      Some(T),
//  }

fn main() {
    let home = IpAddr {
        kind: IpAddrType::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrType::V6,
        address: String::from("::1"),
    };

    route(IpAddrType::V4);

    let home1 = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddrEnum::V6(String::from("::1"));

    let home2 = IpAddrEnum2::V4(127, 0, 0, 1);
    let loopback2 = IpAddrEnum2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // in std lib Options, Some and None are already available and can be used directly
    let some_int = Some(5);
    let some_char = Some('e');
    let absent_int = Option::<i32> = None;

    // Some<T> is not same as T so Some(5) + 3 will throw error at compile time
}
