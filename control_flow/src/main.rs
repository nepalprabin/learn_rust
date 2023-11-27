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

    // Repeating code with loop
    let mut counter = 0;
    let result = loop {
        println!("Entering into loop!");
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("Result is: {result}")
}
