use std::io;

use io::stdin;

fn main() {
    println!("Please enter a temperature (numbers only):");

    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("There was an error reading the terminal");
    
    let inp_tempr: f32 = input.trim().parse().expect("Please enter a number!");
    
    println!("Please enter the temperature to convert FROM [c,f,k]:");
    
    input.clear();
    stdin().read_line(&mut input).expect("There was an error reading the terminal");
    
    let type_initial: char = input.to_lowercase().trim().chars().next().unwrap();//expect("Please enter a single character!");
    
    println!("Please enter the temperature to convert TO [c,f,k]:");
    
    input.clear();
    stdin().read_line(&mut input).expect("There was an error reading the terminal");
    
    let type_desired: char = input.to_lowercase().trim().chars().next().unwrap();//.expect("Please enter a single character!");

    let mut result: f32 = 0f32;

    match type_initial{
        'c' =>{ /*celsius*/
            result = c_to_k(inp_tempr);
        }
        'f' =>{ /*fahrenheit*/
            result = f_to_k(inp_tempr);
        }
        'k' =>{ /*Kelvin*/
            result = inp_tempr;
        }
        _ => {
            println!("uh, something didn't match?");
        }   
    }

    match type_desired{
        'c' =>{ /*celsius*/
            result = k_to_c(result);
        }
        'f' =>{ /*fahrenheit*/
            result = k_to_f(result);
        }
        'k' =>{ /*Kelvin*/
        }
        _ => {
            println!("uh, something didn't match?");
        }   
    }

    println!("{inp_tempr}°{type_initial} -> {result:.2}°{type_desired}");

}

fn c_to_k(c:f32) ->f32{
    c + 273.15
}

fn f_to_k(f:f32) ->f32{
    (f + 459.67)*(5f32/9f32)
}

fn k_to_f(k:f32) ->f32{
    1.8*(k-273f32)+32f32
}

fn k_to_c(k:f32) ->f32{
    k - 273.15
}