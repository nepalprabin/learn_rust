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
    println!("Result is: {result}");

    // loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
        }
    println!("End Count = {count}");

    // for loop
    for age in (1..10).rev() {
        println!("Age: {age}");
    }
}
