use std::io::Read;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

fn main() {
    enable_raw_mode().unwrap();
    for b in std::io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03} \n", b);
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?} \n", b, c)
                }
        
                if c == 'q' {
                    disable_raw_mode().unwrap();
                    break;
                }              
            }
            Err(err) => print!("Error: {}", err)
        }
    }
}
