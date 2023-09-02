use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Green")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(100);
    println!("{:?}", scores);

    let key = String::from("Blue");
    let score = match scores.get(&key) {
        Some(value) => *value,
        None => 0,
    };
    println!("{}: {:?}", key, score);

    println!("------------------");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);

    let test = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in test.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
