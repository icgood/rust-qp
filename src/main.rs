extern crate qp;
use std::io::stdio::stdin;
use qp::ToQP;

fn main() {
    let s = stdin().read_to_end().unwrap();
    println!("{}", s.to_qp(Some(76)));
}
