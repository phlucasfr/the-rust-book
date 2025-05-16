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
    println!("{y}")
}
