fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");

    //invalid operation cannot borrow `*some_string` as mutable, as it is behind a `&` reference

    change(&mut s1);

    //cannot borrow `s` as mutable more than once at a time example
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//invalid operation cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// fn change(some_string: &String) {
//     some_string.push(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
