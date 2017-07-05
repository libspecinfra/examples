extern crate specinfra;

use specinfra::backend;
use specinfra::backend::Backend;

fn main() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("/etc/paasswd");
    match file.mode() {
        Ok(m) => println!("{:o}", m),
        Err(e) => println!("{}", e),
    }
}
