use rand::Rng;

fn chapter_3_5() {
    let number = rand::thread_rng().gen_range(0..10);
    println!("Number is: {}", number);

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is nut divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut iterator = 0;
    let test = 0;
    loop {
        iterator += 1;
        let test = test + 1;
        if iterator == 2 {
            continue;
        }
        println!("iterator: {}, test: {}", iterator, test);
        if iterator > 3 {
            break;
        }
    }
    println!("iterator: {}, test: {}", iterator, test);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let mut number = 0;

    while number <= 15 {
        fizz_buzz(number);
        number += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
        if element >= 25 {
            break;
        }
    }

    for index in 0..a.len() {
        println!("the value is: {}", a[index]);
    }
}

fn fizz_buzz(num: u32) {
    if num % 15 == 0 {
        println!("fizz buzz");
    } else if num % 5 == 0 {
        println!("buzz");
    } else if num % 3 == 0 {
        println!("fizz");
    } else {
        println!("{}", num);
    }
}
