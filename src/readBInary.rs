use std::fs::File;
use std::io::Read;

fn main()
{
    let mut file= File::open("/home/volker/Playground/Rust/src/data.bin").unwrap();

    let mut buf=[0u8;6];

    file.read(&mut buf).unwrap();

    println!("{:?}",buf);

    // use file
}