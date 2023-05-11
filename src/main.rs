use rum::rumload;
use std::env;
use rum::um;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());

    um::handle_input(instructions);
}
