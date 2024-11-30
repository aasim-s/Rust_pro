/*
* A trait defines the functionality a particular type has and can share with other types. We can
* use traits to define shared behavior in an abstract way. We can use trait bounds to specify that
* a generic type can be any type that has certain behavior.
*
* Traits are similar to interfaces in other languages, although with some differences.
*
* A type’s behavior consists of the methods we can call on that type. Different types share the same
* behavior if we can call the same methods on all of those types. Trait definitions are a way to group
* method signatures together to define a set of behaviors necessary to accomplish some purpose.
* */
use std::fmt::Debug;
use std::fmt::Display;

// defining a trait
pub trait Summary {
    // fn summarize(&self) -> String; // just signature
    // can have a default impl
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing the trait for struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by P{} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// implementing the trait for struct
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("MindForge241"),
        content: String::from("frog in a well"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

/*
* we can’t implement external traits on external types. For example, we can’t implement the Display trait
* on Vec<T> within our crate because Display and Vec<T> are both defined in the standard library and aren’t
* local to our crate.
*
* This restriction is part of a property called coherence, and more specifically the orphan rule, so named
* because the parent type is not present. This rule ensures that other people’s code can’t break your code
* and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust
* wouldn’t know which implementation to use.
* */

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking News!! {}", item.summarize());
}
/*
* Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name.
* This parameter accepts any type that implements the specified trait.
* */
// above is simpler form for below
// pub fn notify<T: Summary>(item: &T) {
//   println!("Breaking news! {}", item.summarize());
//}

// Specifying Multiple Trait Bounds with the + Syntax
// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {

// Clearer Trait Bounds with where Clauses
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { // not good
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    4
}

// Returning Types That Implement Traits
fn return_summerizable() -> impl Summary {
    Tweet {
        username: String::from("MindForge241"),
        content: String::from("frog in a well"),
        reply: false,
        retweet: false,
    }
}
// you can only use impl Trait if you’re returning a single type.
// For example, this code that returns either a NewsArticle or a Tweet with the return type
// specified as impl Summary wouldn’t work.
