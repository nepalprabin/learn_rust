fn main() {
    let number = 3;

    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }

    let condition = true;
    let n = if condition {1} else {2};
    println!("The value of n is: {n}");
}
