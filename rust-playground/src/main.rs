#![deny(clippy::all)]

fn main() {
    let home = IPAddress::V4(IPV4 {
        address: "45.68.78.34".to_string(),
    });
    let loopback = IPAddress::V6(IPV6 {
        address: "25.32.24.24".to_string(),
    });
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    println!("Home: {}", home.get_ip());
    println!("Loopback: {}", loopback.get_ip());
}
impl IPAddress {
    fn get_ip(&self) -> String {
        match self {
            IPAddress::V4(ipv4) => ipv4.address.clone(),
            IPAddress::V6(ipv6) => ipv6.address.clone(),
        }
    }
}
#[derive(Debug)]
enum IPAddress {
    V4(IPV4),
    V6(IPV6),
}
#[derive(Debug)]
struct IPV4 {
    address: String,
}
#[derive(Debug)]
struct IPV6 {
    address: String,
}
