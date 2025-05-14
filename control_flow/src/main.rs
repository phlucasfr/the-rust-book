fn main() {
    //if example
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //if bool example
    if number == 3 {
        println!("number was three");
    }

    //else if example
    let number = 6; //shadowing number

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if in a let statement example
    let condition = true;
    let number = if condition { 5 } else { 6 }; //must be the same type

    println!("The value of number is: {}", number);

    //loop example
    // loop {
    //     println!("again!");
    // }

    //returning values from loops example
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("The result is {}", result);

    //loop labe example
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while example
    let mut number = 3;

    while number != 0 {
        println!("Number is: {number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //looping thorough a collection with for example
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //for example
    for element in a {
        println!("the value is: {element}");
    }

    //for with reverse example
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
