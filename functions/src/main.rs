//entry point of a rust program
fn main() {
    println!("Hello, world!");

    another_function(10);
    print_labeled_measurement(5, 'h');

    let five = five();
    println!("The value of five is: {five}");
}

//snake case example with parameter
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

//mutiple parameters example
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//return value example
fn five() -> i32 {
    5
}
