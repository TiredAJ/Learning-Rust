use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main()
{
    let greeting_file_result = File::open("Hello.txt");

    //let greeting_file = match greeting_file_result
    //{
    //    Ok(file) => file,
    //    Err(error) => match error.kind()
    //    {
    //        ErrorKind::NotFound => match File::create("Hello.txt")
    //        {
    //            Ok(fc) => fc,
    //            Err(e) => panic!("Problem creating the file! {:?}", e),
    //        },
    //        other_error =>
    //        {panic!("Problem opening the file: {:?}", other_error);}
    //    },
    //};

    //does a basic version of the above, handling the panic for us
    //let greeting_file = File::open("Hello.txt").unwrap();

    let greeting_file = File::open("Hello.txt")
        .expect("Hello.txt should be included in the project");
}

fn ReadUsernameFromFile() -> Result<String, io::Error>
{
    let UsernameFileResult = File::open("Hello.txt");

    let mut UsernameFile = match UsernameFileResult
    {
        Ok(File) => File,
        Err(e) => return Err(e),
    };

    let mut Username = String::new();

    match UsernameFile.read_to_string(&mut Username)
    {
        Ok(_) => Ok(Username),
        Err(e) => Err(e),
    }
}