
//initial
/*fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} pixels²", area(width1,height1));
}

fn area(width: u32, height: u32) -> u32{
    width*height
}*/

//tuples
/*fn main(){
    let rec1 = (30,50);

    println!("The area of the rectangle is {} pixels²", area(rec1));
}

fn area(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}*/

//struct refactor
/*fn main(){
    let rec1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} pixels²", area(&rec1));

    //the :? tells the macro to use the debug output format
    // :#? adds newlines after each member
    println!("rec1 is {:?}", rec1);
}

#[derive(Debug)] //this allows the struct to be used with the debug output format
struct Rectangle{
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) ->u32{
    rect.width * rect.height
}*/

//some dbg! stuff with structs
//dbg (unlike println!()) takes and returns ownership of an expression
/*fn main(){
    let scale = 2;
    let rec1 = Rectangle{
        width: dbg!(30*scale),
        height: 50,
    };

    //we don't want dbg!() to take ownership of rec1, so we give 
    //it a reference instead
    dbg!(&rec1);
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}*/

//struct methods
//calling a method of a referenced object doesn't require changing the notation
//as it's done automatically :o
fn main(){
    let rec1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle{
        width: 60,
        height: 45,
    };

    let sq1 = Rectangle::square(30);

    println!("The area of the rectangle is {} pixels²", rec1.area());

    if rec1.width(){
        println!("Rec1 has a nonzero width of {}", rec1.width);
    };

    println!("Can rect1 hold rect2? {}", rec1.can_hold(&rec2));
    println!("Can rect1 hold rect3? {}", rec1.can_hold(&rec3));
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

//implementation block
//every function in here is an associated function, but only those with &self
//parameters are methods.
impl Rectangle{
    //&self within an impl represents self: &self. it still uses & because
    //it borrows
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn width(&self) -> bool{
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    //methods are usually used for constructors. This creates and returns a new
    //square rectangle struct instance. To call these, we need to use :: instead of
    //dot notation
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
