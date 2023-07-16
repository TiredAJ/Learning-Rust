//chapter 5!

//structs basics
/*fn main() {
    let mut user1 = User{
        active: true,
        username: String::from("Jane Doe"),
        email: String::from("JD@Somethin.co.uk"),
        sign_in_count: 1,
    };

    user1.email = String::from("JDNew@Something.co.uk");

    /*let user2 = User{
        active: true,
        username: user1.username,
        email: String::from("JDNewNew@Something.co.uk"),
        sign_in_count: user1.sign_in_count,
    };*/ //can be written easier as:

    let user2 = User{
        email: String::from("JDNewNew@Something.co.uk"),
        ..user1 //this autofills any *remaining* fields with data from user1
    }

    //because the string username is moved from user1, it's no longer useable 
    //due to move rules. If we were to also assign a new username, it'd be able
    //to copy active and sing_in_count because they are copyable.
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,   //because the fn parameters are the same as the struct names
        email,      //we don't need to write the struct member:
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}*/

//Tuple Structs
//the only difference between tuples and tuple structs is that an argument struct
//must match the parameter struct, so origin can't be used in a function expecting
//a Colour struct even though they are both composed of 3 32bit ints.
/*fn main(){
    let black = Colour(0,0,0);
    let origin = Point(0,0,0);
}

struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);
*/

//unit structs
//supposedly this is useful for implementing traits on a type without storing data.
//it's discussed more in chapter 10.
fn main(){
    let subject = AlwaysEqual;
}

struct AlwaysEqual;