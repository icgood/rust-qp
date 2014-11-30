extern crate qp;
use qp::ToQP;

fn main() {
    let s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt \r\nin culpa qui officia deserunt mollit anim id est laborum.";
    println!("{}", s.as_bytes().to_qp(Some(76)));
}
