fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}")

    //invalid operation cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // change(&s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//invalid operation cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// fn change(some_string: &String) {
//     some_string.push(", world");
// }
