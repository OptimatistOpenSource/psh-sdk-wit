#[allow(clippy::all)]
#[allow(dead_code)]
mod bindings;

use bindings::op;

fn main() {
    println!("{}", op::system::version::os_version().unwrap());
}
