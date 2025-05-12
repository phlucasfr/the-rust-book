fn main() {
    // SCALAR SESSION
    //integer types
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize

    //literals
    // Decimal	98_222
    // Hex	0xff
    // Octal	0o77
    // Binary	0b1111_0000
    // Byte (u8 only)	b'A'

    //float types
    // f32
    // f64

    //numeric operations
    // +
    // -
    // *
    // /
    // %

    //boolean type
    // true
    // false

    // character type
    // char

    // COMPOUND SESSION
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    months.iter().for_each(|&month| println!("{}", month));

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    a.iter().for_each(|element: &i32| println!("{}", element));

    let a = [3; 5];
    a.iter().for_each(|element: &i32| println!("{}", element)); //output 3 3 3 3 3

    let first = a[0];
    let second = a[1];
    println!("the value of first is: {first}");
    println!("the value of second is: {second}");
}
