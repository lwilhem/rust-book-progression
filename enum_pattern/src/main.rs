#[derive(Debug)]
enum IPAddrKind {
  V4(String),
  V6(String),
}

fn main() {
  println!("Hello, world!");
  let home = IPAddrKind::V4(String::from("127.0.0.1"));
  let loopback = IPAddrKind::V6(String::from("::1"));

  router(&home);
  router(&loopback)
}

fn router(ip_kind: &IPAddrKind) {
  println!("{:#?}", ip_kind)
}
