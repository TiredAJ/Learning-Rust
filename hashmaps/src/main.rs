use std::collections::HashMap;

fn main() {
    //creating a hashmap & accessing values
    /*let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /*let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //.get() returns an Option<&V>, .copied() returns Option<i32>
    //and .unwrap_or() sets score to 0 if there isn't an entry
    //for the key*/

    //iterating
    for (key, value) in &scores {
        println!("{key}:{value}");
    }*/

    //ownership
    /*let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //from here, field_name & field_value are invalid because
    //they've both been moved into the hashmap*/

    //overwriting a value
    /*let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);

    //this overwrites any entry with the key "Blue"
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); //prints "{"Blue": 25}"*/

    //adding a key & value if a key doesn't exist
    /*let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    //.entry returns an Entry enum, .or_insert returns a mut
    //reference to the value if the key exists. if not, it inserts
    //the parameter as the new value for this key and returns a 
    //mutable reference to the new value.

    println!("{:?}", scores);
    //prints {"Yellow": 50, "Blue": 10}*/

    //updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    //this counts how many times a word appears
    //the .split_whitespace() returns an iterator over sub-slices
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    //this prints {"world": 2, "hello": 1, "wonderful": 1}

}
