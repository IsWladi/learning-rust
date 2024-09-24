fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let words = String::from("make some salad");
    let words_literal = "do some salad"; // this is equivalent  to: &str
    println!("First word: {}", first_word(&words));
    println!("First word literal: {}", first_word(words_literal));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
