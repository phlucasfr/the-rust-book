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

}
