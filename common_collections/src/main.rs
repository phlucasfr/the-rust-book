use core::num;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(0);

    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); 
    match third {
        Some(num) => println!("The third element is {num}"),
        None => println!("There's no third element")
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // v.push(6);
    println!("The first element is: {first}");

    let v = vec![100, 32, 57];
    for i in &v  {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v  {
        *i += 50;
        println!("{i}");
    }

}
