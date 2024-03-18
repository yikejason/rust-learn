// 定义枚举
// enum IpAddKind {
//     v4,
//     v6,
// }

// fn main() {
//     let four = IpAddKind::v4;
//     let six = IpAddKind::v6;
//     route(four);
//     route(six);
//     route(IpAddKind::v6);
// }

// fn route(ip_kind: IpAddKind) {}

// 将数据附加到枚举的变体中
// enum IpAddKind {
//     v4,
//     v6,
// }
// struct IpAddr {
//     kind: IpAddKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddKind::v4,
//         address: String::from("127.0.0.1"),
//     };
//     let loopback = IpAddr {
//         kind: IpAddKind::v6,
//         address: String::from("::1"),
//     }
// }
// 为枚举定义方法
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Move { x, y } => println!("{}", x),
            _ => println!("No output"),
        }
    }
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);

    m.call();
}
