use std::net::IpAddr;

fn main() {
    {
        let home = IPAddr {
            kind: IPAddrKind::v4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IPAddr {
            kind: IPAddrKind::v6,
            address: String::from("::1"),
        };

        println!("home is {:#?}", home);
        println!("loopback is {:#?}", loopback);
    }

    {
        let home = IPAddress::v4(127, 0, 0, 1);
        let loopback = IPAddress::v6(String::from("::1"));

        println!("home is {:#?}", home);
        println!("loopback is {:#?}", loopback);
    }
}

#[derive(Debug)]
enum IPAddrKind {
    v4,
    v6,
}

#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

#[derive(Debug)]
enum IPAddress {
    // 枚举成员就与值相关联
    v4(u8, u8, u8, u8),
    v6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
