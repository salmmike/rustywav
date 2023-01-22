use rustywav;

fn main() {
    let path = String::from("/home/mike/Code/MPKPlayer/resources/test.wav");
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

}
