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

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("the value of s is {r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("the value of s is {r2}");

    //cannot borrow `s` as mutable because it is also borrowed as immutable
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}

//lifetime missing example, this function's return type contains a borrowed value, but there is no value
//for it to be borrowed from
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }// Here, s goes out of scope, and is dropped, so its memory goes away.
// Danger!

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
