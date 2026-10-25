use std::{error::Error, fs, path::Path};

#[derive(Debug, Clone, Copy)]
pub(crate) struct MemoryBus {
    address_space: [u8; 0x10000],
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            address_space: [0; 0x10000],
        }
    }

    pub fn load_rom(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let rom = fs::read(path)?;
        let rom_len = rom.len();

        if rom_len > 0x8000 {
            return Err(format!(
                "ROM size too big ({} bytes long, maximum size 32768 bytes). ROM path: {}",
                rom_len,
                path.display()
            )
            .into());
        }

        self.address_space[..rom_len].copy_from_slice(&rom);

        Ok(())
    }

    pub fn write(&mut self, address: u16, value: u8) {
        // NOTE - temporary serial port
        match address {
            0xFF02 if value == 0x81 => {
                eprint!("{}", self.address_space[0xFF01] as char);
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
            _ => {}
        }
        // TODO - check for forbidden addresses
        self.address_space[address as usize] = value;
    }

    pub fn read(&self, address: u16) -> u8 {
        // TODO - check for forbidden reads perhaps
        match address {
            0xFF44 => 0x90, // NOTE - hardcoded for gameboy doctor while PPU not finished
            _ => self.address_space[address as usize]
        }
    }
}
