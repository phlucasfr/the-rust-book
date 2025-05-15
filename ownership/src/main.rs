fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        println!("the value of s is {s}")

        // do stuff with s
    } // this scope is now over, and s is no longer valid
}
