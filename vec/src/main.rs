fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    for i in &mut v {
        *i += 2;
    }
    println!("{:?}", v);

    match v.get(10) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            SpreadSheetCell::Int(int) => println!("{}", int),
            SpreadSheetCell::Float(float) => println!("{}", float),
            SpreadSheetCell::Text(text) => println!("{}", text),
        }
    }
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
