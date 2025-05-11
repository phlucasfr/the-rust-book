fn main() {
    //default imutable variable example
    let z = 10;
    println!("The value o z is {z}");

    //mutable variable example
    let mut x = 5;
    println!("The value o x is {x}");
    x = 6;
    println!("The value o x is {x}");

    //const example
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconder are {THREE_HOURS_IN_SECONDS}");

    //shadowing example
    let shadowing = 5;
    let shadowing = shadowing + 1;

    {
        let shadowing = shadowing * 2;
        println!("The value of shadowing in the inner scope is: {shadowing}");
    }

    println!("The value of shadowing is: {shadowing}");
}
