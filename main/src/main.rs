use clap::{clap_app, crate_version};
use pulldown_cmark::{html::push_html, Event, Parser};
fn main() {
    let clap = clap_app!(main=> 
    (version:crate_version!())
    (author:"The Ramaitre")
    (about:"render markdown as you like")
    (@arg input: +required "Sets the input file"))
    .get_matches();
    let mut res = String::new();
    let infile = std::fs::read_to_string(clap.value_of("input").unwrap()).expect("could not read file");
    let ps = Parser::new(&infile);

    let ps:Vec<Event> = ps.into_iter().collect();
    for p in &ps{
        println!("{:?}", p)
    }
    push_html(&mut res, ps.into_iter());
    println!("{}", res);
}
