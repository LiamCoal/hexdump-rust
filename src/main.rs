use std::env::*;
use std::fs::*;
use std::io::*;
use std::*;
//use math::round;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    let mut file: File = File::open(&args[1])?;
    let meta = metadata(&args[1])?;
    for i in 0..meta.len() {
        let mut buf = [0; 1];
        let _ = file.read(&mut buf)?;
        if i % 16 == 0 {
            print!("\n");
            print!("{:08X?}", i);
        }

        if i % 8 == 0 {
            print!(" ");
        }

        print!("{:02X?} ", buf[0]);
    }
    println!();
    Ok(())
}