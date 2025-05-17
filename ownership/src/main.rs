fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        println!("the value of s is {s}")

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}");

    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("{s}");

        // do stuff with s
    } // this scope is now over, and s is no
    // longer valid

    let x = 5;
    let y = x;
    println!("{y}");

    //invalidated reference error example
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{s1}, world!");

    let mut s = String::from("hello");
    //here we have a warn value it's overwritten before being read
    s = String::from("ahoy");

    println!("{s}, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    //valid because integers that have a known size at compile time are stored entirely on the stack
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    test();
    test2();
    test3();
}

//returning tuple example
fn test3() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn test2() {
    let s1 = gives_ownership(); // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn test() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // because i32 implements the Copy trait,
    // x does NOT move into the function,
    println!("{}", x); // so it's okay to use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
