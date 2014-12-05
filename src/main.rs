extern crate qp;
use std::io::stdio::{stdin, stdout};
use qp::ToQPFile;

fn main() {
    stdin().to_qp(&mut stdout(), Some(76));
}
