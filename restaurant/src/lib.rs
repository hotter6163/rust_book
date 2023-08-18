use rand::Rng;
use std::{
    collections::HashMap,
    fmt::Result,
    io::{self, Write},
};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    let secret_number = rand::thread_rng().gen_range(1..101);
}
