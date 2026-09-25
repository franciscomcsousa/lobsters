use std;
use std::error::Error;
use std::process::exit;

mod memory_map {
    use std::{error::Error, fs, path::Path};

    #[derive(Debug)]
    pub(crate) struct AddressSpace {
        address_map: [u8; 0xFFFF],
    }

    impl AddressSpace {
        pub fn new() -> Self {
            AddressSpace {
                address_map: [0; 0xFFFF],
            }
        }

        pub fn load_rom(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
            let rom = fs::read(path)?;
            let rom_len = rom.len();

            if rom_len > 0x8000 {
                return Err(format!(
                    "ROM size too big ({} bytes long, maximum 32768 bytes). ROM path: {}",
                    rom_len,
                    path.display()
                )
                .into());
            }

            self.address_map[..rom_len].copy_from_slice(&rom);

            Ok(())
        }
    }
}

mod graphics {

    pub(crate) struct Screen {
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <ROM_PATH>");
        exit(1);
    }

    let path = std::path::Path::new(&args[1]);

    // TODO - check if its a valid ROM extention and check integrity of ROM (somehow?)
    if path.is_file() {
        println!("Using ROM on path {}", path.display());
    } else {
        eprintln!("Invalid ROM path: {}", path.display());
        exit(1);
    }

    let mut space = memory_map::AddressSpace::new();
    space.load_rom(path)?;

    println!("Address Space: {:X?}", space);

    

    Ok(())
}
