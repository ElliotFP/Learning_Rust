fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(" world!");
    println!("The length of '{}' is {}.", s1, s1.len());

    let word = first_word(&s1);
    println!("The first word of '{}' is {}.", s1, &s1[..word]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
