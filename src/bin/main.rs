use rustywav;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        println!("{}", arg);
    }
    if args.len() > 1 {
        let path = &args[1];
        match rustywav::read_wav(&path) {
            Ok(wav) => {
            println!("{}", (wav.header.riff[0] as char));
            println!("{}", (wav.header.riff[1] as char));
            println!("{}", (wav.header.riff[2] as char));
            println!("{}", (wav.header.riff[3] as char));
            },
            Err(err) => {
                println!("Read wav file {} failed", path);
                println!("Error: {}", err);
            }
        }
    } else {
        println!("Add wav path as argument!");
    }

}
