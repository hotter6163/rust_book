fn main() {
    for i in 0..10 {
        println!("Fibonacci {} is: {}", i, fibonacci_dp(i));
    }
}

// fn celsius_to_fahrenheit(celsius: f64) -> f64 {
//     celsius * 1.8 + 32.0
// }

// fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
//     (fahrenheit - 32.0) / 1.8
// }

// fn fibonacci(num: u64) -> u64 {
//     if num == 0 {
//         0
//     } else if num == 1 {
//         1
//     } else {
//         fibonacci(num - 1) + fibonacci(num - 2)
//     }
// }

fn fibonacci_dp(num: usize) -> u64 {
    let mut dp = vec![0; num + 1];
    for i in 0..=num {
        if i == 1 {
            dp[i] = 1;
        } else if i > 1 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
    }
    *dp.last().unwrap_or(&0)
}
