extern crate ferris_says;

use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main()
{
    let stdout = stdout();
    let out = b"Hello dude!";
    let width = 12;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}