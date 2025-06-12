#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

struct QuitMessage; //unit struct
struct MoveMessage {
    //standard struct
    x: i32,
    y: i32,
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 8);
    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(home);
    dbg!(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');

    let absent_number: Option<i32> = None; //require type annotation

    let x: i8 = 5;
    let mut y: Option<i8> = Some(10);

    //cannot add `Option<i8>` to `i8`
    // let sum = x + y;

    let sum = x + match y.as_mut() {
        Some(val) => *val,
        None => 0,
    };

    println!("The sum of x and y is {}", sum);
}

fn route(ip_kind: IpAddrKind) {
    println!("The kind of ip is {:?}", ip_kind);
}
