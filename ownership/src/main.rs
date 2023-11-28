fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{} world", s2);

    // Variables and Data Interacting with Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s1={}, s2={}", s3, s4);

}
