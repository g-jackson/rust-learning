fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    /*
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum IpAddr {
    V4(String),
    V6(String),
}

/*
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/
enum IpAddrKind {
    V4,
    V6,
}
