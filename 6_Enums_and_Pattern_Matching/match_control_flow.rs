#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(String),
}

//match pattern can bind to a value if needed
// =matches are exhaustive as the arms should cover all the posiible values
//if any one of the possible coin values is not catered for, compiler will throw error
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(text) => {
            println!("your lucky message is: {text:?}!");
            25
        }
    }
}

//matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn reroll() {}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(moves: u8) {}

fn main() {
    value_in_cents(Coin::Quater(String::from("get lost bud")));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // catch-all patterns  (using some place holder like default in switch case)
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    // can use _ instead of other if we have no use for the value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    // can use unity value tuple if you dont wanna do anything
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
    
    let config_max = Some(3u8);
    match config_max {
    	Some(max) => println!("The maximum is configured to be {max}"),
    	_ => (),
    }
    // can use if an let key words to make match less verbose
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }
    // can add an else if needed
}
