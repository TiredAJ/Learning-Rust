use std::collections::*;

fn main() {
    //empty vector
    /*let v: Vec<i32> = Vec::new();*/

    //initialise vector
    /*let v = vec![1,2,3];*/

    //updating vectors
    /*let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);*/

    //reading elements
    /*let v = vec![1,2,3,4,5];

    //gets a reference to the 3rd element
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    //this returns an option so we can check if there's 
    //a third option
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("There is no third element"),
    }*/

    //accessing out of bounds
    /*let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; //this causes a panic
    let does_not_exist = v.get(100); //this just returns None*/

    //references
    /*let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
    //this would prevent the program from compiling because
    //you can't hold a reference and change the size of the vector
    //because the vector may need to be moved in the heap for 
    //more space*/

    //iterating over a vector (reading)
    /*let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }*/

    //iterating over a vector (writing)
    /*let mut v = vec![100, 32, 57];

    for i in &mut v {
        //to change the value, we dereference i with *
        *i += 50;
    }*/

    //using an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];



} //<- v goes out of scope and is dropped along with all it's elements