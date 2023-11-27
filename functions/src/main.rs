fn main() {
    println!("Hello, world!");
    alarm_function(5, "hours");
}

fn alarm_function(unit: i32, unit_label: &str) {
    println!("Wake up after: {unit} {unit_label}");
}
