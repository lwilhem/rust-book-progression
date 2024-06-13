fn main() {
  let four = IpAddrKind::V4(String::from("127.0.0.1"));
  let six = IpAddrKind::V6(String::from(":1"));

  router(&four);
  router(&six)
}

fn router(ip_addr: &IpAddrKind) {
  println!("{:#?}", ip_addr)
}

#[derive(Debug)]
#[allow(dead_code)]
enum IpAddrKind {
  V4(String),
  V6(String),
}
