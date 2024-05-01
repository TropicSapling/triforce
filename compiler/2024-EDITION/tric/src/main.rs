use std::{fs, io::Error};
use culpa::throws;

#[throws]
fn main() {
    let text = fs::read_to_string("../postcard.tri")?;

    println!("");

    dbg!(text);
}
