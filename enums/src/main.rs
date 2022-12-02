fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        v4,
        v6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr{
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::v6,
        address: String::from("..1"),
    };

    println!("{:?}", home);
}
