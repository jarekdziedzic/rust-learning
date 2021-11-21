use crate::IpAddr::{IPv4, IPv6};

enum IpAddr {
    IPv4(u8,u8,u8,u8),
    IPv6(String),
}

impl IpAddr {
    fn ping(&self) -> u32 {
        match &self {
            IpAddr::IPv4(o1,o2,o3,o4) => {
                println!("Pinging {}.{}.{}.{} ...", o1, o2, o3, o4);
            },
            IpAddr::IPv6(s) => {
                println!("Pinging {} ...", s)
            }
        }
        1
    }

}

fn main() {
    IPv4(127,0,0,1).ping();
    IPv6(String::from("::1")).ping();
    let p = "hello";
    let s = format!("[blah] {}", p);

    print!("{}", s);
}
