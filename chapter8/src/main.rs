mod function;

use crate::function::{
    question1::{calc_mean, calc_medium, calc_mode},
    question2::pig_latin,
    question3::Company,
};

fn main() {
    println!("Question1");
    let numbers = vec![1, 3, 5, 10, 4, 7, 3, 12, 2];
    println!("vec: {:?}", numbers);
    println!("mean: {}", calc_mean(&numbers));
    println!("medium: {}", calc_medium(&numbers));
    println!("mode: {}", calc_mode(&numbers));

    println!("-----------------------");

    println!("Question2");
    println!("{}", pig_latin(&String::from("first")));
    println!("{}", pig_latin(&String::from("apple")));

    println!("-----------------------");

    println!("Question3");
    let mut company = Company::new();
    company.call("Add Sally to Engineering");
    company.call("Add Amir to Sales");
    if let Some(list) = company.get_department_employees("Engineering") {
        println!("{:?}", list);
    }
    println!("{:?}", company.get_all_employees());
}
