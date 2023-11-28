fn main() {
    let s = String::from("hello");

    let length = calculate_length(&s);                  // &s is string referencing.. 
                                                        // This provides reference to the string value and does not take ownership of it 

    println!("The length of '{}' is {}.", s, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
