use std;
use std::error::Error;
use std::process::exit;

mod memory_bus;
mod ppu;
mod cpu;

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

    let mut space = memory_bus::MemoryBus::new();
    space.load_rom(path)?;

    println!("Address Space: {:X?}", space);

    let mut screen = ppu::Screen::new(); 
    screen.render_loop();

    Ok(())
}
