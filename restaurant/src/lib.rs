mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist() {}
        fn seat_at_bench() {}
    }
    mod serving{
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    //only public fields will be public in a public struct
    pub struct Breakfast {
        pub toast : String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //a public enum will have all of it's variants as public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

//this is re-exporting and allows whatever's calling this code to access
//hosting
pub use crate::front_of_house::hosting;

mod customer {
    //this brings hosting into scope, this needs to be in a module 
    //to be used
    use crate::front_of_house::hosting;
    use crate::back_of_house;

    pub fn eat_at_restaurant(){
        //absolute path
        crate::front_of_house::hosting::add_to_waitlist();
        
        //relative path, doesn't work with the "use crate..."
        //front_of_house::hosting::add_to_waitlist();
        
        //in-scope path
        hosting::add_to_waitlist();
        
        //order breakfast in the summer with rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        
        //change our mind about what bread
        meal.toast = String::from("Wheat");
        
        println!("I'd like {} toast please", meal.toast);
        
        //this won't compile because seasonal fruit is private
        //meal.seasonal_fruit = String::from("blueberries");
        
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}