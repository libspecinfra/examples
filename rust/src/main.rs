extern crate specinfra;

use specinfra::backend;
use specinfra::backend::Backend;

#[allow(unused_must_use)]
fn main() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("/etc/passwd");
    file.mode()
        .map_err(|e| println!("{}", e))
        .map(|m| println!("{:o}", m));
}
