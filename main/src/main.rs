use clap::{clap_app, crate_version};
fn main() {
    let _clap = clap_app!(main=> 
    (version:crate_version!())
    (author:"The Ramaitre")
    (about:"render markdown as you like")
    (@arg input: +required "Sets the input file"))
    .get_matches();
    println!("Hello, world!");
}
