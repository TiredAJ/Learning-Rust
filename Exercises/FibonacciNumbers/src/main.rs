use std::io;

fn main() {
    println!("please enter how many fibonacci numbers you'd like: ");

    //assigns the mutable n to a new string
    let mut n = String::new();

    //reads a line from the terminal and puts it into a mutable buffer, otherwise error message
    io::stdin().read_line(&mut n).expect("Failed to readline");

    //shadow n from a string, parsing into an unsigned 32, if it fails, display error message
    let n: u32 =  n.trim().parse().expect("Please enter a number!");

    let mut x: u32 = 0;
    let mut y: u32 = 1;
    let mut r: u32 = 0;

    for _i in 0u32..n{ //i isn't used, but loop it from 0 (unsigned 32bit to n)
        
        println!("{r}");

        r = x + y;        
        x = y;
        y = r;
    }

}
