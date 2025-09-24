use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let mut writer = BufWriter::new(stdout());
    say("Hello, world!", 24, &mut writer).unwrap();
}
