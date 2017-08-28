extern crate specinfra;

use specinfra::backend::ssh::SSHBuilder;

fn main() {
    let s = SSHBuilder::new();
    let b = s.hostname("localhost").finalize().unwrap();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("/etc/passwd");
    match file.mode() {
        Ok(m) => println!("{:o}", m),
        Err(e) => println!("{}", e),
    }
}
