#[macro_use]
extern crate validator_derive;
extern crate validator;
extern crate serde;

mod with_crate;
mod from_scratch;

fn main() {
    with_crate::play();
    println!("\n\n\n");
    from_scratch::play();
}
