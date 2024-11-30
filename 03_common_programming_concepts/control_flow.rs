fn main() {
    let condition = true;
    let x = if condition { 6 } else { 3 };

    // if else statments
    if x < 5 {
        println!("number is less than 5");
    } else if x == 5 {
        println!("number is equal to 5");
    } else {
        println!("number is greater than 5");
    }

    // loops need break statements
    let mut counter = 0;
    let sum = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("the sum is {sum}");

    // loops can have labels for disambiguity
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // conditional loop with while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println! {"the value is: {element}"};
    }

    // for loop with an in and rev
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("SECOND LIFTOFF!!!");
}
