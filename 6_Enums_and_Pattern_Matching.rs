// basic enum syntax
enum IpAddrType {
    V4,
    V6,
}
// let four = IpAddrType::V4;

fn route(ip_kind: IpAddrKind) {}
// route(IpAddrkind::V4);

// using enum with struct
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
let home = IpAddr {
    kind: ipAddrType::V4,
    address: String::from("127.0.0.1"),
};
let loopback = IpAddr {
    kind: IpAddrType::V6,
    address: String::from("::1"),
};

// here it will be more consise to use enum directly instead of in a strut
// can directly put value in enum field
enum IpAddr_enum {
    V4(String)
    V6(String)
}
let home = IpAddr_enum::V4(String::from("127.0.0.1"));
let loopback = IpAddr_enum::V6(String::from("::1"));

// another better way with enum
enum IpAddr_enum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr_enum2::V4(127, 0, 0, 1);
let loopback = IpAddr_enum2::V6(String::from("::1"));

//bit more complicated enum
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
// can defined methods on enum just like structs
impl Message {
    fn call(&self) {
        // some functionality
    }
}
let m = Message::Write(String::from("hello"));
m.call();
