/* HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function(SipHash),
*  which determines how it places these keys and values into memory
*  */

use std::collections::HashMap;

fn main() {
    // creating a new hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 30);

    // retrive data from hash map
    let team = String::from("Blue");
    // get returns a Option<&v>, copied makes it Option<v>,
    // unwrap_or gives value if Some or a default value if None
    let _score = scores.get(&team).copied().unwrap_or(0);

    // iterate over a pair
    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    /* For types that implement the Copy trait, like i32, the values are copied into the hash map.
     * For owned values like String, the values will be moved and the hash map will be the owner
     * of those values
     * */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them gives compiler error

    /* If we insert references to values into the hash map, the values won’t be moved into the hash map.
     * The values that the references point to must be valid for at least as long as the hash map is valid.
     * */

    // over write the value
    scores.insert(String::from("Blue"), 25);
    // get if exist or insert
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(47);
}
