use std::io;

//basics
/* 
fn main(){
    //doesn't work because s1 is moved to s2
    let s1 = String::from("Hello");
    let s2 = s1;
    
    println!("{s1}");
    
    //this works because s2 is a copy of s1, rather than s1 moved
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    
    println!("s1: {s1}, s2: {s2}");
}*/

//ownership & functions
//some_string in t_o is s moved, so s no longer is the owner of the data
//but some_int has the data copied from x
/*
fn main(){
    let s = String::from("Hello!");
    
    takes_ownership(s);
    
    let x = 5;
    
    makes_copy(5);
}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_int: i32){
    println!("{some_int}");
}*/

//return values & scope
/*
fn main() {
    //some_string from g_o() passes ownership over to s1
    let s1 = gives_ownership();

    let s2 = String::from("Helo");

    //s2 passes ownership to a_string in t_a_g_b()
    let s3 = takes_and_gives_back(s2);
} //s3 & s1 are dropped here, s2 has no ownership so nothing happens to it

fn gives_ownership() -> String{
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string;
}*/

//references & borrowing
//s in c_l() becomes a reference to s1's data, rather than taking ownership, so
//when s goes out of scope, s1 doesn't loose it's value
/*
fn main(){
    let s1 = String::from("Heya");

    let len = calculate_length(&s1);

    println!("{len}");
}

fn calculate_length(s: &String) -> usize{
    s.len();
}*/

//mutable references
//because s and some_string are marked mut, change can alter the data 
//some_string is borrowing. But now s has a mutable reference, it can have
//no more references to it. this prevents data races
//both r1 and r2 can mutably reference s because r1 goes out of scope before s2
//exists.
/*fn main(){
    let mut s = String::from("Helo");

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;

    change(&s);
}

fn change(some_string: &mut String) -> String{
    some_string.push_str(", ddaear");
}*/

//mutable & immutable referenes
//when you have immutable references, you can't also have a mutable reference
//but multiple immutable references are allowed.
/*
fn main(){
    let mut s = String::from("Hello");
    
    let r1 = &s; //fine
    let r2 = &s; //fine
    let r3 = &mut s; //NOT FINE, WTF DUDE?
}*/


//reference scope
//references cease to exist after their last usage, so the below code works fine
//because r1 and r2 aren't used after the first println!()
/*
fn main()
{
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1},{r2}");

    let r3 = &mut s;
    println!("{r3}");
}*/

//dangling references?
//this won't compile due to lifetimes (something for later?) but also because
//s goes out of scope when it tries to return a reference to it, so the
//reference goes to nothing, preventing compilation. To get around this, you
//can just return s instead of a reference so ownership is moved
/*
fn main(){
    let reference_to_nothing = dangle();
}

fn dangle() -> &String{
    let s = String::from("Hello");

    &s
}*/

//recaperino
//- at any given time a variable can have only one mutable reference or
//  several immutable references
//- references must be valid




//
//Slices!
//

//string slices
//allows for specifying sections of a referenced string
/*
fn main(){
    let s = String::from(" Hello there pal, how ya doin'?");

    let fw = first_word(&s);

    //s.clear(); //this would cause an error

    println!("[{fw}]");
}

//fn first_word(s: &String) -> &str{
fn first_word(s: &str) -> &str{ //changed to s: &str allows it to accept string and
//slices
    let bytes = s.as_bytes();

    //enumerate takes the return value of iter and returns a tuple with
    //the index and reference to the value. I think the b' ' is to 
    //check item against the *byte* form of a whitespace character
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];//should return the first word
        }
    }

    &s[..] //returns the whole string
}*/


//other slices
//general slices can be used on things like arrays
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    //idfk how to use assert_eq, the doc doesn't talk about it
    assert_eq!(slice, &[2,3]);
}