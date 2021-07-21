use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);

    let mut contents = File::open(filename);

    let mut placeholder = 0;
    let mut buffer = Vec::new();
    contents.read_to_end(&mut buffer);

    while placeholder < buffer.len(){
        match buffer[placeholder]
        {
            0x00 => println!("NOP"),
            0x02 => println!("STAX B")
            //etc etc
        }
        placeholder += 1;
    }
    return 0;

}
