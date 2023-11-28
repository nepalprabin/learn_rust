fn main() {
    // Ownership and Functions
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // value of s is moved to the function.. and s is not longer valid


    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x moved to the function
                                    // but x is a Copy, so x is still valid
    println!("After function: {}", x);



}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}