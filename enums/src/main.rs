fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
}
// defining method for an enum as you would do in struct 
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call (&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
