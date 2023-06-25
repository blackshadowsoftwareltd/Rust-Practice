#![deny(clippy::all)]

fn main() {
    let home = IPAddress::V4(IPV4::Address("25.43.64.12".to_string()));
    let loopback = IPAddress::V6(IPV6::Address("25.32.24.24".to_string()));
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    println!("Home: {}", home.get_ip());
    println!("Loopback: {}", loopback.get_ip());
}
impl IPAddress {
    fn get_ip(&self) -> String {
        match self {
            IPAddress::V4(ipv4) => match ipv4 {
                IPV4::Address(ip) => ip.clone(),
            },
            IPAddress::V6(ipv6) => match ipv6 {
                IPV6::Address(ip) => ip.clone(),
            },
        }
    }
}
#[derive(Debug)]
enum IPAddress {
    V4(IPV4),
    V6(IPV6),
}
#[derive(Debug)]
enum IPV4 {
    Address(String),
}
#[derive(Debug)]
enum IPV6 {
    Address(String),
}
