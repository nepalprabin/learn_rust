fn main() {
    println!("Hello, world!");
    alarm_function(5, "hours");
    let y = return_value_fn();
    println!("The value from return_value_fn() is: {y}");
}

fn alarm_function(unit: i32, unit_label: &str) {
    println!("Wake up after: {unit} {unit_label}");
}

fn return_value_fn() -> i32 {
    10
}