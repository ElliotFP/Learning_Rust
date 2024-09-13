mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
        fn seat_at_table() {
            println!("seat_at_table");
        }
    }

    mod serving {
        fn take_order() {
            println!("take_order");
        }
        fn serve_order() {
            println!("serve_order");
        }
        fn take_payment() {
            println!("take_payment");
        }
    }
}

fn serve_order() {
    println!("serve_order");
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    use crate::back_of_house::Breakfast;

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
