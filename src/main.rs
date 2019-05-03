#[macro_use]
extern crate validator_derive;
extern crate itertools;
extern crate serde;
extern crate validator;

mod from_scratch;
mod with_crate;

fn main() {
    with_crate::play();
    println!("\n\n\n");
    from_scratch::play();
}
