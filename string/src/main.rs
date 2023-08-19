fn main() {
    let mut s = String::from("foo");
    println!("{}", s);
    s.push_str("bar");
    println!("{}", s);
    let s2 = String::from("baz");
    s.push_str(&s2);
    println!("{}, {}", s, s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{}", s);
    println!("{:?}", s.chars());
}
