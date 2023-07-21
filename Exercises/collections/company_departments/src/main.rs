use std::collections::HashMap;
use std::io;

use std::io::stdin;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();
    let mut command: Vec<String> = Vec::new();

    departments.insert(
        String::from("Sales"),
        vec![
            String::from("April"),
            String::from("Hillary"),
            String::from("Charles"),
        ]
    );

    display_department(&String::from("Sales"), &departments);

    loop {
        println!("Please enter \"[-command]\" for commands");

        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        for word in input.split_whitespace() {
            command.push(word.to_string());
        }

        if &command[0] == "add" {
            
        }
        else if &command[0] == "view"{
            if &command[1] == "all" {
                //show all by department
            }
            else {
                //look for specific department
            }
        }

    }

}

fn add_to_department(employee: &String, department: &String, map: &mut HashMap<String, Vec<String>>){

}

fn display_department(department: &String, map: &HashMap<String, Vec<String>>){
    let emps: Option<(String, Vec<String>)> map.get(department);
    
    match emps {
        Some(emps) => {
            println!("{key}");
            for emp in value {
                println!("\t{emp}");
            }
        },
        None => println!("Sorry, that department doesn't exist"),
    }
}