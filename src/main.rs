use std;
use std::error::Error;
use std::process::exit;

mod cpu;
mod memory_bus;
mod ppu;

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <ROM_PATH>");
        exit(1);
    }

    let path = std::path::Path::new(&args[1]);

    // TODO - check if its a valid ROM extention and check integrity of ROM (somehow?)
    // if path.is_file() {
    //     println!("Using ROM on path {}", path.display());
    // } else {
    //     eprintln!("Invalid ROM path: {}", path.display());
    //     exit(1);
    // }

    let mut space = memory_bus::MemoryBus::new();
    space.load_rom(path)?;

    // println!("Starting Address Space: {:X?}", space);

    // let mut screen = ppu::Screen::new();
    // screen.render_loop();
    
    let mut cpu = cpu::Cpu::new(space);
    let mut total_cycles : u64 = 0;
    loop {
        let cycles = cpu.step();
        if cycles == 0 {
            break;
        }        
        total_cycles += cycles as u64;
    }

    // println!("Final Address Space: {:X?}", space);

    Ok(())
}
