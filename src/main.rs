use std::fs::File;
use std::io::{self, Error, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("zzz.txt")?;
    // let mut out = File::open("out.txt");
    let mut buffer = [0u8; 1];
    let mut warn = false;

    while file.read(&mut buffer)? != 0 {
        let byte = buffer[0];

        if (byte >> 7) & 1 == 0 {
            warn = true;
            continue;
        }

        let mut length = 0;
        for i in (4..=6).rev() {
            if (byte >> i) & 1 == 0 {
                break;
            }
            length += 1;
        }

        let shift = 2 + length;
        let mut cur = (byte << shift) >> shift;

        if cur == 0 {
            warn = true;
            continue;
        }

        for _ in 0..length {
            if file.read(&mut buffer)? == 0 {
                break;
            }
            let byte = buffer[0];
            cur <<= 6;
            cur += (byte << 2) >> 2;
        }

        println!("{}", cur);
    }

    if warn {
        println!("Error occured while parsing")
    }

    Ok(())
}
