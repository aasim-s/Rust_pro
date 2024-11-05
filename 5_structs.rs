struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

//tuple structs have self name but no field names, they have field type tho
struct Color(i32, i32, i32);// (r, b, g)
struct Point(i32, i32, i32);// (x, y, z)

//tuples with no fields are called unit-like structs, they are useful to implement traits on some type
struct AlwaysEqual;

//just like any expression can return an instance of struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // same as username: username,
        email, // same as email: email,
        sign_in_count: 1,
    }
}

#[derive(Debug)] // for prtining using println or dbg
struct Rectangle {
    width: u32,
    height: u32,
}
// methods are simialr to functions but they are defined within the context of  struct/enum/triat obj...
//also called associated functions
impl Rectangle {
    fn area(&self) -> u32 { // short for self: &self
        self.width * self.height
    }

    fn can_hold(&self, &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // is associated function but not a method as no self parameter in input
    // are called using ::
    fn square(size: u32) -> self { 
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // the whole strut needs to be mutaible, cannot have a single mut field
    let mut user1 = User {
        active: true,
        username: String::from("name123"),
        email: String::("name@mail.com"),
        sign_in_count: 1,
    };
    //each struct is its own type, even if the fields within might have same type
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let condition = AlwaysEqual;

    let user2 = build_user("name@mail.com", "name345");

    // create a struct instance from another instance. this is called struct update syntax.
    // this moves the data from user2 to user1 so we can no longer use user1 as a whole again after this.
    // we could still use user1 as a whole if the username was also asinged a new string value
    let user3 = User {
        active: user2.active,
        username: user2.username,
        email: String::from("name@mail.com"),
        sign_in_count = user2.sign_in_count,
    }
    // shorter syntax
    let user4 = User {
        email: String::("name@mail.com"),
        ..user1 //should come last to specify that any remaining fields should get their value from user1
    }

    user1.email = String::("fullname@mail.com");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    }
    println!("Area of the rectangle is {}". rect1.area())
    let sq = Rectangle::square(13);

}

