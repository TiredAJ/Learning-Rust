
fn main() {
    //creates an empty mutable string
    /*let mut s = String::new();*/

    //to_string
    /*let data = "initial contents";

    let s = data.to_string();
    //this also works, but i don't like it tbh
    //let s = "initial contents".to_string();*/

    //::from()
    /*let s = String::from("initial contents");*/

    //appending to a string
    /*let mut s = String::from("foo");
    s.push_str("bar");*/

    //using a string slice after appending
    /*let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");*/

    //push vs push_str
    /*let mut s = String::from("lo");
    s.push('l');    //pushes a character
    s.push_str("olololol"); //pushes a string*/

    //concatentation
    /*let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 has been moved here, cannot be used.
    //this is because the + function looks like:
    //fn add(self, s: &str) -> string {//--trim--}
    //with this, we can only add a string reference to a string,
    //we can't add two strings together. Further, the add function
    //causes the compiler to coerce the &String into a &str*/

    //more complex concatenation
    /*let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    //this macro's function uses reference so it doesn't take
    //ownership. It works similarly to println!() but returns
    //a string instead of writing to the terminal*/

    //indexing into strings
    /*let s1 = String::from("hello");
    let h = s1[0]; //this doesn't work because of how strings work:
    //because strings are utf-8 encoded and stored in vec<u8>, 
    //not all characters take up the same memory.
    //"Hola" would take up 4B, but "Здравствуйте" takes up 24B
    //because each cyrillic letter there takes up 2B each
    //further... (bytes, scalar vals & grapheme clusters)
    //the Hindi word “नमस्ते” in the Devanagari script is stored 
    //in a vec<u8> as : 
    //[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 
    //141, 224, 164, 164,224, 165, 135]
    //as unicode scalara values (rust's char type) they look like:
    //['न', 'म', 'स', '्', 'त', 'े'], 6 chars, but no. 4 and 6
    //aren't characters but diacritics
    //as a grapheme cluster they look like:
    //["न", "म", "स्", "ते"]
    //hence why you can't index a string*/

    //slicing strings
    /*let hello = "Здравствуйте";
    let s = &hello[0..4];
    //this takes the first 4B, and because each of these
    //characters are 2B, it'll return "Зд".
    //if you were to try [0..1], rust would panic*/

    //iterating over strings
    /*for c in "Зд".chars() {
        println!("{c}");
    } //this prints "З" then "д"
    
    //alternatively
    for b in "Зд".bytes() {
        println!("{b}");
    }this prints "208", "151", "208" then "180"*/
}