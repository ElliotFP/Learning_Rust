fn main() {
    let mut s1 = String::from("hello");
    let s2 = " world";

    s1.push_str(s2);
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s3 = String::from("hello");
    let s4 = String::from("world");

    let s5 = s3.clone() + &s4;

    println!("s5: {}", s5);
    println!("s3: {}", s3);

    let s6 = String::from("hello");
    let s7 = format!("{}-{}-{}", s6, s4, s5);
    println!("s7: {}", s7);

    let s8 = "你的妈";
    let s9 = &s8[0..3];
    println!("s9: {}, len: {}", s9, s8.len());

    for c in s8.chars() {
        println!("{}", c);
    }

    for b in s8.bytes() {
        println!("{}", b);
    }
}

use std::collections::HashMap;

fn track() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score: {}", score);
}
