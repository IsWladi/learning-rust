fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    let len2 = calculate_length_with_ref(&s2);

    println!("The length of '{s2}' is {len}.");
    println!("(with ref) The length of '{s2}' is {len2}.");

    let mut mutable_s_by_ref = String::from("hello with ref");
    change(&mut mutable_s_by_ref);
    println!("{}", mutable_s_by_ref);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", changed!");
}
